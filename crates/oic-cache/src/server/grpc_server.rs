//! gRPC 服务器，将 CacheService 映射到 Cache API。

use crate::cache::Cache;
use crate::server::proto;
use proto::cache_service_server::{CacheService, CacheServiceServer};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::instrument;

/// gRPC CacheService 实现
pub struct CacheServiceImpl {
    cache: Arc<Cache>,
}

impl CacheServiceImpl {
    pub fn new(cache: Arc<Cache>) -> Self {
        Self { cache }
    }
}

#[tonic::async_trait]
impl CacheService for CacheServiceImpl {
    #[instrument(skip(self))]
    async fn get(
        &self,
        request: Request<proto::GetRequest>,
    ) -> Result<Response<proto::GetResponse>, Status> {
        let key = request.into_inner().key;
        match self.cache.get(&key).await {
            Ok(Some(data)) => Ok(Response::new(proto::GetResponse {
                data: data.to_vec(),
                found: true,
            })),
            Ok(None) => Ok(Response::new(proto::GetResponse {
                data: vec![],
                found: false,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    #[instrument(skip(self))]
    async fn set(
        &self,
        request: Request<proto::SetRequest>,
    ) -> Result<Response<proto::SetResponse>, Status> {
        let req = request.into_inner();
        const DEFAULT_TTL: i64 = 360;
        let ttl = if req.ttl_seconds != 0 {
            req.ttl_seconds
        } else {
            DEFAULT_TTL
        };
        match self
            .cache
            .set_with_ttl(
                req.key,
                req.data,
                "application/octet-stream".to_string(),
                ttl,
            )
            .await
        {
            Ok(()) => Ok(Response::new(proto::SetResponse { success: true })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    #[instrument(skip(self))]
    async fn get_statistics(
        &self,
        _request: Request<proto::Empty>,
    ) -> Result<Response<proto::StatsResponse>, Status> {
        let s = self.cache.statistics().await;
        Ok(Response::new(proto::StatsResponse {
            total_requests: s.total_requests as i64,
            hits: s.hits as i64,
            misses: s.misses as i64,
            hit_rate: s.hit_rate,
            total_entries: s.total_entries as i64,
        }))
    }

    #[instrument(skip(self))]
    async fn invalidate_namespace(
        &self,
        request: Request<proto::InvalidateRequest>,
    ) -> Result<Response<proto::InvalidateResponse>, Status> {
        let ns = request.into_inner().namespace;
        match self.cache.invalidate_namespace(&ns).await {
            Ok(n) => Ok(Response::new(proto::InvalidateResponse {
                invalidated_count: n as i32,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

/// 启动 gRPC 服务器
pub async fn start_grpc_server(
    cache: Arc<Cache>,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = CacheServiceImpl::new(cache);
    let svc = CacheServiceServer::new(service);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;
    Ok(())
}

/// 使用已有 TcpListener 启动 gRPC 服务器（用于测试等场景）
pub async fn start_grpc_server_with_listener(
    cache: Arc<Cache>,
    listener: tokio::net::TcpListener,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = CacheServiceImpl::new(cache);
    let svc = CacheServiceServer::new(service);
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve_with_incoming(incoming)
        .await?;
    Ok(())
}
