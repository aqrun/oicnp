//! 缓存服务化：Redis Protocol 与 gRPC 服务端。

pub mod grpc_server;
pub mod proto {
    tonic::include_proto!("cache");
}
pub mod redis_server;
pub mod resp;

pub use grpc_server::{start_grpc_server, start_grpc_server_with_listener, CacheServiceImpl};
pub use redis_server::RedisServer;
