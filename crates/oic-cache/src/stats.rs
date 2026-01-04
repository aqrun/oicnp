use crate::metadata::StatsInfo;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_requests: u64,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub total_entries: usize,
    pub memory_usage_mb: f64,
    pub disk_usage_gb: f64,
}

/// 全局统计计数器
#[derive(Clone)]
pub struct Statistics {
    total_requests: Arc<AtomicU64>,
    hits: Arc<AtomicU64>,
    misses: Arc<AtomicU64>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            hits: Arc::new(AtomicU64::new(0)),
            misses: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub fn record_hit(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.hits.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_miss(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.misses.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_statistics(&self, total_entries: usize, memory_mb: f64, disk_gb: f64) -> CacheStatistics {
        let total = self.total_requests.load(Ordering::Relaxed);
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        
        CacheStatistics {
            total_requests: total,
            hits,
            misses,
            hit_rate,
            total_entries,
            memory_usage_mb: memory_mb,
            disk_usage_gb: disk_gb,
        }
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}

/// 更新统计信息
pub fn update_stats(
    stats: &mut StatsInfo,
    read_time_us: u64,
    bytes_served: u64,
) {
    stats.access_count += 1;
    stats.last_accessed = chrono::Utc::now().timestamp();
    stats.hit_count += 1;
    stats.total_bytes_served += bytes_served;
    
    // 更新平均读取时间（简单移动平均）
    if stats.access_count == 1 {
        stats.avg_read_time_us = read_time_us;
    } else {
        stats.avg_read_time_us = (stats.avg_read_time_us * 9 + read_time_us) / 10;
    }
    
    // 更新访问频率（基于时间窗口）
    let now = chrono::Utc::now().timestamp();
    let time_diff = (now - stats.last_accessed).max(1) as f64;
    stats.access_frequency = stats.access_count as f64 / time_diff;
}

/// 创建初始统计信息
pub fn create_stats() -> StatsInfo {
    let now = chrono::Utc::now().timestamp();
    StatsInfo {
        access_count: 0,
        last_accessed: now,
        access_frequency: 0.0,
        hit_count: 0,
        avg_read_time_us: 0,
        total_bytes_served: 0,
    }
}

