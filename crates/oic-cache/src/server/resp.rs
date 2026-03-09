//! RESP (Redis Serialization Protocol) 解析与编码。

use std::io::{Cursor, Read};

/// RESP 解析错误
#[derive(Debug, Clone)]
pub struct RespError(pub String);

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RESP error: {}", self.0)
    }
}

impl std::error::Error for RespError {}

/// 从字节流解析一条完整的 RESP 消息（数组或单条），返回解析到的参数列表和消费的字节数。
/// 客户端通常发送 *N\r\n$len\r\n... 形式的数组。
pub fn parse_resp(data: &[u8]) -> Result<(Vec<Vec<u8>>, usize), RespError> {
    if data.is_empty() {
        return Err(RespError("empty input".into()));
    }
    let mut cur = Cursor::new(data);
    let first = read_byte(&mut cur)?;
    match first {
        b'*' => {
            let n = read_int_until_crlf(&mut cur)?;
            let mut args = Vec::with_capacity(n as usize);
            let _start = cur.position() as usize;
            for _ in 0..n {
                let b = read_byte(&mut cur)?;
                if b != b'$' {
                    return Err(RespError("expected bulk string $".into()));
                }
                let len = read_int_until_crlf(&mut cur)?;
                let mut buf = vec![0u8; len as usize];
                cur.read_exact(&mut buf).map_err(|e| RespError(e.to_string()))?;
                let mut crlf = [0u8; 2];
                cur.read_exact(&mut crlf).map_err(|e| RespError(e.to_string()))?;
                if crlf != [b'\r', b'\n'] {
                    return Err(RespError("expected \\r\\n after bulk string".into()));
                }
                args.push(buf);
            }
            let consumed = cur.position() as usize;
            Ok((args, consumed))
        }
        _ => Err(RespError(format!("unexpected RESP type: {}", first as char))),
    }
}

fn read_byte<R: Read>(r: &mut R) -> Result<u8, RespError> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b).map_err(|e| RespError(e.to_string()))?;
    Ok(b[0])
}

fn read_int_until_crlf<R: Read>(r: &mut R) -> Result<i64, RespError> {
    let mut buf = Vec::new();
    loop {
        let mut b = [0u8; 1];
        if r.read(&mut b).map_err(|e| RespError(e.to_string()))? == 0 {
            return Err(RespError("unexpected EOF".into()));
        }
        if b[0] == b'\r' {
            let mut n = [0u8; 1];
            r.read_exact(&mut n).map_err(|e| RespError(e.to_string()))?;
            if n[0] != b'\n' {
                return Err(RespError("expected \\n after \\r".into()));
            }
            break;
        }
        buf.push(b[0]);
    }
    let s = String::from_utf8(buf).map_err(|_| RespError("invalid UTF-8 in length".into()))?;
    s.trim()
        .parse::<i64>()
        .map_err(|_| RespError("invalid integer".into()))
}

// ---------- 编码 ----------

/// `+OK\r\n`
pub fn encode_simple_string(s: &str) -> Vec<u8> {
    let mut v = b"+".to_vec();
    v.extend_from_slice(s.as_bytes());
    v.extend_from_slice(b"\r\n");
    v
}

/// `-ERR message\r\n`
pub fn encode_error(msg: &str) -> Vec<u8> {
    let mut v = b"-".to_vec();
    v.extend_from_slice(msg.as_bytes());
    v.extend_from_slice(b"\r\n");
    v
}

/// `:123\r\n`
pub fn encode_integer(n: i64) -> Vec<u8> {
    let mut v = b":".to_vec();
    v.extend_from_slice(n.to_string().as_bytes());
    v.extend_from_slice(b"\r\n");
    v
}

/// `$5\r\nhello\r\n`，空用 `$0\r\n\r\n`
pub fn encode_bulk_string(data: &[u8]) -> Vec<u8> {
    let mut v = b"$".to_vec();
    v.extend_from_slice(data.len().to_string().as_bytes());
    v.extend_from_slice(b"\r\n");
    v.extend_from_slice(data);
    v.extend_from_slice(b"\r\n");
    v
}

/// `$-1\r\n` 表示 nil
pub fn encode_null_bulk_string() -> Vec<u8> {
    b"$-1\r\n".to_vec()
}

/// `$0\r\n\r\n` 空字符串
pub fn encode_empty_bulk_string() -> Vec<u8> {
    b"$0\r\n\r\n".to_vec()
}

/// RESP3 map 值：字符串或整数（HELLO 响应中 proto/id 须为整数）。
pub enum Resp3MapVal<'a> {
    Str(&'a str),
    Int(i64),
}

/// RESP3 map：%n\r\n 后接 n 对 key/value；value 为 bulk string 或 integer。
/// 用于 HELLO 3 响应，使 redis-rs 0.31 / bb8-redis 能完成握手。
pub fn encode_resp3_map(pairs: &[(&str, Resp3MapVal<'_>)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"%");
    out.extend_from_slice(pairs.len().to_string().as_bytes());
    out.extend_from_slice(b"\r\n");
    for (k, v) in pairs {
        out.extend_from_slice(&encode_bulk_string(k.as_bytes()));
        match v {
            Resp3MapVal::Str(s) => out.extend_from_slice(&encode_bulk_string(s.as_bytes())),
            Resp3MapVal::Int(n) => out.extend_from_slice(&encode_integer(*n)),
        }
    }
    out
}
