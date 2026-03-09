//! Redis Protocol (RESP) 服务器，将 Redis 命令映射到 Cache API。

use crate::cache::Cache;
use crate::server::resp;
use bytes::Bytes;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};

const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
const DEFAULT_TTL_SECONDS: i64 = 360;
const BUF_SIZE: usize = 4096;

/// Redis 协议服务器
pub struct RedisServer {
    cache: Arc<Cache>,
}

impl RedisServer {
    pub fn new(cache: Arc<Cache>) -> Self {
        Self { cache }
    }

    pub async fn run(&self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        self.run_with_listener(listener).await
    }

    /// 使用已有 TcpListener 运行（用于测试等场景，可绑定随机端口）
    pub async fn run_with_listener(&self, listener: TcpListener) -> std::io::Result<()> {
        let addr = listener.local_addr()?;
        tracing::info!("Redis protocol server listening on {}", addr);
        loop {
            let (socket, peer) = match listener.accept().await {
                Ok(x) => x,
                Err(e) => {
                    tracing::warn!("accept error: {}", e);
                    continue;
                }
            };
            let cache = Arc::clone(&self.cache);
            tokio::spawn(async move {
                if let Err(e) = handle_client(socket, cache).await {
                    tracing::debug!("redis client {} error: {}", peer, e);
                }
            });
        }
    }
}

async fn handle_client(socket: TcpStream, cache: Arc<Cache>) -> std::io::Result<()> {
    let (mut reader, writer) = socket.into_split();
    let mut writer = BufWriter::new(writer);
    let mut buf = vec![0u8; BUF_SIZE];
    let mut read_pos = 0usize;

    loop {
        let n = reader.read(&mut buf[read_pos..]).await?;
        if n == 0 {
            break;
        }
        read_pos += n;
        let data = &buf[..read_pos];

        let (args, consumed) = match resp::parse_resp(data) {
            Ok(x) => x,
            Err(_e) => {
                tracing::debug!("resp parse error: {:?}, buf = {:?}", _e, &data[..read_pos]);

                if read_pos >= BUF_SIZE {
                    let _ = writer
                        .write_all(&resp::encode_error("ERR request too large"))
                        .await;
                    let _ = writer.flush().await;
                    read_pos = 0;
                }
                continue;
            }
        };

        let cmd_args: Vec<String> = args
            .iter()
            .map(|a| String::from_utf8_lossy(a).to_string())
            .collect();
        tracing::debug!("redis raw args: {:?}", cmd_args);
        // 便于集成测试排查：无 tracing 时也能在 stderr 看到首包
        if args.first().map(|c| c.as_slice()) == Some(b"HELLO") {
            eprintln!("[oic-cache] received HELLO, args={:?}", cmd_args);
        }

        read_pos -= consumed;
        if read_pos > 0 {
            buf.copy_within(consumed.., 0);
        }

        if args.is_empty() {
            let _ = writer
                .write_all(&resp::encode_error("ERR empty command"))
                .await;
            let _ = writer.flush().await;
            continue;
        }

        let cmd = std::str::from_utf8(&args[0]).unwrap_or("");
        let response = dispatch(&cache, cmd, &args).await;
        if let Err(e) = writer.write_all(&response).await {
            tracing::debug!("write error: {}", e);
            break;
        }
        if let Err(e) = writer.flush().await {
            tracing::debug!("flush error: {}", e);
            break;
        }
    }
    Ok(())
}

async fn dispatch(cache: &Cache, cmd: &str, args: &[Vec<u8>]) -> Vec<u8> {
    match cmd {
        c if c.eq_ignore_ascii_case("HELLO") => {
            // RESP3 握手：客户端发 HELLO 3，服务端返回 map（proto/id 为整数），便于 redis-rs 0.31 / bb8-redis 建连
            let _proto = args.get(1).and_then(|a| std::str::from_utf8(a).ok());
            let pairs: &[(&str, resp::Resp3MapVal)] = &[
                ("server", resp::Resp3MapVal::Str("redis")),
                ("version", resp::Resp3MapVal::Str("0.0.0")),
                ("proto", resp::Resp3MapVal::Int(3)),
                ("id", resp::Resp3MapVal::Int(0)),
                ("mode", resp::Resp3MapVal::Str("standalone")),
                ("role", resp::Resp3MapVal::Str("master")),
            ];
            resp::encode_resp3_map(pairs)
        }
        c if c.eq_ignore_ascii_case("PING") => resp::encode_simple_string("PONG"),
        c if c.eq_ignore_ascii_case("SELECT") => {
            // 兼容 Redis 客户端在连接后发送的 SELECT 命令。
            // 当前实现不区分逻辑 DB，因此忽略 DB 索引并统一返回 OK。
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'SELECT'");
            }
            // 尝试解析 DB 索引，但结果忽略（仅用于校验格式）。
            let _ = std::str::from_utf8(&args[1])
                .unwrap_or("0")
                .parse::<i64>();
            resp::encode_simple_string("OK")
        }
        c if c.eq_ignore_ascii_case("CLIENT") => {
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'CLIENT'");
            }
            let sub = std::str::from_utf8(&args[1]).unwrap_or("");
            if sub.eq_ignore_ascii_case("SETINFO") {
                // 忽略后续参数，直接返回 OK 即可
                return resp::encode_simple_string("OK");
            }
        
            // 其他 CLIENT 子命令目前不支持
            resp::encode_error("ERR unknown CLIENT subcommand")
        }
        c if c.eq_ignore_ascii_case("GET") => {
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'GET'");
            }
            let key = String::from_utf8_lossy(&args[1]).to_string();
            match cache.get(&key).await {
                Ok(Some(data)) => resp::encode_bulk_string(&data),
                Ok(None) => resp::encode_null_bulk_string(),
                Err(e) => resp::encode_error(&format!("ERR {}", e)),
            }
        }
        c if c.eq_ignore_ascii_case("SET") => {
            if args.len() < 3 {
                return resp::encode_error("ERR wrong number of arguments for 'SET'");
            }
            let key = String::from_utf8_lossy(&args[1]).to_string();
            let value = Bytes::from(args[2].clone());
            let mut ttl = DEFAULT_TTL_SECONDS;
            let mut i = 3;
            while i + 1 < args.len() {
                let opt = std::str::from_utf8(&args[i]).unwrap_or("");
                if opt.eq_ignore_ascii_case("EX") {
                    if let Ok(secs) = std::str::from_utf8(&args[i + 1]).unwrap_or("0").parse::<i64>() {
                        ttl = secs;
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            match cache
                .set_with_ttl(key, value, DEFAULT_CONTENT_TYPE.to_string(), ttl)
                .await
            {
                Ok(()) => resp::encode_simple_string("OK"),
                Err(e) => resp::encode_error(&format!("ERR {}", e)),
            }
        }
        c if c.eq_ignore_ascii_case("DEL") => {
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'DEL'");
            }
            let key = String::from_utf8_lossy(&args[1]).to_string();
            match cache.invalidate(&key).await {
                Ok(()) => resp::encode_integer(1),
                Err(e) => resp::encode_error(&format!("ERR {}", e)),
            }
        }
        c if c.eq_ignore_ascii_case("EXISTS") => {
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'EXISTS'");
            }
            let key = String::from_utf8_lossy(&args[1]).to_string();
            let exists = cache.exists(&key).await;
            resp::encode_integer(if exists { 1 } else { 0 })
        }
        c if c.eq_ignore_ascii_case("FLUSHALL") => match cache.clear().await {
            Ok(()) => resp::encode_simple_string("OK"),
            Err(e) => resp::encode_error(&format!("ERR {}", e)),
        },
        c if c.eq_ignore_ascii_case("INVALIDATE_NS") => {
            if args.len() < 2 {
                return resp::encode_error("ERR wrong number of arguments for 'INVALIDATE_NS'");
            }
            let ns = String::from_utf8_lossy(&args[1]).to_string();
            match cache.invalidate_namespace(&ns).await {
                Ok(n) => resp::encode_integer(n as i64),
                Err(e) => resp::encode_error(&format!("ERR {}", e)),
            }
        }
        c if c.eq_ignore_ascii_case("STATS") => {
            let stats = cache.statistics().await;
            let s = format!(
                "hits:{} misses:{} total_requests:{} hit_rate:{} total_entries:{}",
                stats.hits,
                stats.misses,
                stats.total_requests,
                stats.hit_rate,
                stats.total_entries
            );
            resp::encode_bulk_string(s.as_bytes())
        }
        _ => resp::encode_error("ERR unknown command"),
    }
}
