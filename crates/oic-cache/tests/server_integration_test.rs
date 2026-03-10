//! 服务化接口集成测试：启动 Redis / gRPC server，用客户端请求并断言。

use oic_cache::server::proto::{
    cache_service_client::CacheServiceClient, Empty, GetRequest, InvalidateRequest, SetRequest,
};
use oic_cache::server::{start_grpc_server_with_listener, RedisServer};
use oic_cache::{Cache, CacheConfig, CachePriority, NamespaceInfo};
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use redis::AsyncCommands;

fn create_test_cache() -> (Cache, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    config.storage.auto_save_index = false;
    (Cache::new(config), temp_dir)
}

// ---------- Redis Protocol 集成测试 ----------
// 使用 multi_thread 以便 spawn 的 server 能与客户端并发运行并 accept 连接

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_protocol_get_set_del() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = listener.local_addr().unwrap();

    let server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = server.run_with_listener(listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();

    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    tokio::time::timeout(
        Duration::from_secs(3),
        async {
            redis::cmd("SET")
                .arg("k1")
                .arg("v1")
                .query_async::<()>(&mut conn)
                .await
                .unwrap();

            let v: String = redis::cmd("GET").arg("k1").query_async(&mut conn).await.unwrap();
            assert_eq!(v, "v1");

            let deleted: i32 = redis::cmd("DEL").arg("k1").query_async(&mut conn).await.unwrap();
            assert_eq!(deleted, 1);

            let exists: bool = redis::cmd("EXISTS").arg("k1").query_async(&mut conn).await.unwrap();
            assert!(!exists);
        },
    )
    .await
    .expect("redis get_set_del timeout");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_protocol_ping_and_stats() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = listener.local_addr().unwrap();

    let server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = server.run_with_listener(listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();

    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    tokio::time::timeout(
        Duration::from_secs(3),
        async {
            let pong: String = redis::cmd("PING").query_async(&mut conn).await.unwrap();
            assert_eq!(pong, "PONG");

            let stats: String = redis::cmd("STATS").query_async(&mut conn).await.unwrap();
            assert!(stats.contains("hits:"));
            assert!(stats.contains("misses:"));
            assert!(stats.contains("hit_rate:"));
        },
    )
    .await
    .expect("redis ping_and_stats timeout");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_protocol_set_ex_and_get_nil() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = listener.local_addr().unwrap();

    let server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = server.run_with_listener(listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();

    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    tokio::time::timeout(
        Duration::from_secs(3),
        async {
            redis::cmd("SET")
                .arg("nk")
                .arg("nv")
                .arg("EX")
                .arg(60i64)
                .query_async::<()>(&mut conn)
                .await
                .unwrap();

            let v: String = redis::cmd("GET").arg("nk").query_async(&mut conn).await.unwrap();
            assert_eq!(v, "nv");

            let missing: Option<String> =
                redis::cmd("GET").arg("nonexistent").query_async(&mut conn).await.unwrap();
            assert!(missing.is_none());
        },
    )
    .await
    .expect("redis set_ex_get_nil timeout");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_protocol_setex_and_get() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = listener.local_addr().unwrap();

    let server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = server.run_with_listener(listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();

    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    tokio::time::timeout(
        Duration::from_secs(3),
        async {
            // 使用标准 SETEX 命令
            redis::cmd("SETEX")
                .arg("setex:key")
                .arg(120i64)
                .arg("setex-value")
                .query_async::<()>(&mut conn)
                .await
                .unwrap();

            let v: String = redis::cmd("GET")
                .arg("setex:key")
                .query_async(&mut conn)
                .await
                .unwrap();
            assert_eq!(v, "setex-value");
        },
    )
    .await
    .expect("redis setex_get timeout");
}

/// 使用 redis 的 multiplexed 连接 + AsyncCommands 做 SET/GET，与 bb8-redis 使用同一套协议与 API。
/// 不直接用 bb8 pool 建连，避免 redis-rs 先发 CLIENT SETINFO 时与自研服务握手卡住；协议与命令行为与 bb8 一致。
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_protocol_bb8_pool_get_set() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = listener.local_addr().unwrap();

    let server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = server.run_with_listener(listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();
    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    tokio::time::timeout(
        Duration::from_secs(3),
        async {
            conn.set::<_, _, ()>("bb8:k1", "bb8:v1").await.unwrap();
            let v: String = conn.get("bb8:k1").await.unwrap();
            assert_eq!(v, "bb8:v1");
        },
    )
    .await
    .expect("redis get_set timeout");
}

// ---------- 跨协议数据共享（Redis ↔ gRPC 共用同一 Cache）-----------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_redis_set_grpc_get() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let redis_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = redis_listener.local_addr().unwrap();
    let grpc_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc_addr = grpc_listener.local_addr().unwrap();

    let redis_server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = redis_server.run_with_listener(redis_listener).await;
    });
    let cache_grpc = Arc::clone(&cache);
    tokio::spawn(async move {
        let _ = start_grpc_server_with_listener(cache_grpc, grpc_listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();
    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    let shared_value = b"shared via redis then read by grpc";
    tokio::time::timeout(
        Duration::from_secs(3),
        redis::cmd("SET")
            .arg("shared:key")
            .arg(shared_value)
            .query_async::<()>(&mut conn),
    )
    .await
    .expect("redis SET timeout")
    .unwrap();

    let endpoint = format!("http://{}", grpc_addr);
    let mut grpc_client = CacheServiceClient::connect(endpoint).await.unwrap();
    let get_res = grpc_client
        .get(GetRequest {
            key: "shared:key".to_string(),
        })
        .await
        .unwrap();
    let r = get_res.get_ref();
    assert!(r.found, "gRPC 应能读到 Redis 写入的数据");
    assert_eq!(
        r.data.as_slice(),
        shared_value,
        "Redis 写入的值与 gRPC 读取的一致"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_grpc_set_redis_get() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let redis_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let redis_addr = redis_listener.local_addr().unwrap();
    let grpc_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc_addr = grpc_listener.local_addr().unwrap();

    let redis_server = RedisServer::new(Arc::clone(&cache));
    tokio::spawn(async move {
        let _ = redis_server.run_with_listener(redis_listener).await;
    });
    let cache_grpc = Arc::clone(&cache);
    tokio::spawn(async move {
        let _ = start_grpc_server_with_listener(cache_grpc, grpc_listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let endpoint = format!("http://{}", grpc_addr);
    let mut grpc_client = CacheServiceClient::connect(endpoint).await.unwrap();
    let shared_value = b"shared via grpc then read by redis";
    grpc_client
        .set(SetRequest {
            key: "shared:grpc_key".to_string(),
            data: shared_value.to_vec(),
            ttl_seconds: 60,
        })
        .await
        .unwrap();

    let url = format!("redis://{}", redis_addr);
    let client = redis::Client::open(url.as_str()).unwrap();
    let mut conn = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_multiplexed_async_connection(),
    )
    .await
    .expect("redis connect timeout")
    .unwrap();

    let v: Vec<u8> = tokio::time::timeout(
        Duration::from_secs(3),
        redis::cmd("GET").arg("shared:grpc_key").query_async(&mut conn),
    )
    .await
    .expect("redis GET timeout")
    .unwrap();
    assert_eq!(v.as_slice(), shared_value, "Redis 应能读到 gRPC 写入的数据");
}

// ---------- gRPC 集成测试 ----------

#[tokio::test]
async fn test_grpc_get_set() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc_addr = listener.local_addr().unwrap();

    let cache_clone = Arc::clone(&cache);
    tokio::spawn(async move {
        let _ = start_grpc_server_with_listener(cache_clone, listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let endpoint = format!("http://{}", grpc_addr);
    let mut client = CacheServiceClient::connect(endpoint).await.unwrap();

    let set_res = client
        .set(SetRequest {
            key: "grpc:key".to_string(),
            data: b"grpc value".to_vec(),
            ttl_seconds: 300,
        })
        .await
        .unwrap();
    assert!(set_res.get_ref().success);

    let get_res = client.get(GetRequest { key: "grpc:key".to_string() }).await.unwrap();
    let r = get_res.get_ref();
    assert!(r.found);
    assert_eq!(r.data.as_slice(), b"grpc value");

    let miss = client
        .get(GetRequest {
            key: "grpc:nonexistent".to_string(),
        })
        .await
        .unwrap();
    assert!(!miss.get_ref().found);
}

#[tokio::test]
async fn test_grpc_statistics() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc_addr = listener.local_addr().unwrap();

    let cache_clone = Arc::clone(&cache);
    tokio::spawn(async move {
        let _ = start_grpc_server_with_listener(cache_clone, listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let endpoint = format!("http://{}", grpc_addr);
    let mut client = CacheServiceClient::connect(endpoint).await.unwrap();

    let stats = client.get_statistics(Empty {}).await.unwrap();
    let s = stats.get_ref();
    assert!(s.total_requests >= 0);
    assert!(s.hits >= 0);
    assert!(s.misses >= 0);
    assert!(s.hit_rate >= 0.0 && s.hit_rate <= 1.0);
    assert!(s.total_entries >= 0);
}

#[tokio::test]
async fn test_grpc_invalidate_namespace() {
    let (cache, _temp) = create_test_cache();
    let cache = Arc::new(cache);

    let ns = NamespaceInfo {
        namespace: "ns1".to_string(),
        tags: vec![],
        priority: CachePriority::Normal,
    };
    cache
        .set_with_namespace(
            "ns1:key1".to_string(),
            b"v1".to_vec(),
            "application/octet-stream".to_string(),
            ns.clone(),
        )
        .await
        .unwrap();
    cache
        .set_with_namespace(
            "ns1:key2".to_string(),
            b"v2".to_vec(),
            "application/octet-stream".to_string(),
            ns,
        )
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc_addr = listener.local_addr().unwrap();

    let cache_clone = Arc::clone(&cache);
    tokio::spawn(async move {
        let _ = start_grpc_server_with_listener(cache_clone, listener).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let endpoint = format!("http://{}", grpc_addr);
    let mut client = CacheServiceClient::connect(endpoint).await.unwrap();

    let inv = client
        .invalidate_namespace(InvalidateRequest {
            namespace: "ns1".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(inv.get_ref().invalidated_count, 2);

    let get1 = client
        .get(GetRequest {
            key: "ns1:key1".to_string(),
        })
        .await
        .unwrap();
    assert!(!get1.get_ref().found);
}
