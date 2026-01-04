# SWR (Stale-While-Revalidate) 机制说明

## 核心问题解答

### 1. SWR 需要保存两份数据吗？

**答案：不需要，只保存一份数据。**

SWR 的工作原理：
- **只保存一份数据**：缓存中只存储一份数据（新鲜数据或过期数据）
- **不复制数据**：当数据过期时，不会创建副本
- **复用过期数据**：如果启用 SWR 且在 `max_stale_seconds` 范围内，直接返回已过期的同一份数据

**数据存储结构：**
```
缓存中只有一份数据：
┌─────────────────┐
│  Key: "user:1"  │
│  Data: {...}    │  ← 只有这一份数据
│  expires_at: T  │
└─────────────────┘
```

当数据过期后：
```
┌─────────────────┐
│  Key: "user:1"  │
│  Data: {...}    │  ← 同一份数据，但标记为过期
│  expires_at: T  │  ← 已过期，但仍在 max_stale 范围内
│  [STALE]        │
└─────────────────┘
```

### 2. 数据过期不会清除吗？

**答案：会清除，但有条件。**

数据清除的时机：

#### 情况 1：立即清除（禁用 SWR 或超过 max_stale）
```rust
// 在 get() 方法中
if is_expired && !is_stale_acceptable {
    // 过期且不能作为 stale 返回，立即删除
    self.invalidate(key).await;  // ← 立即删除
    return Ok(None);
}
```

#### 情况 2：延迟清除（启用 SWR 且在 max_stale 范围内）
```rust
// 在 get() 方法中
if is_expired && is_stale_acceptable {
    // 过期但在 max_stale 范围内，返回 stale 数据
    // 数据不会被立即删除，可以继续使用
    return Ok(Some(stale_data));
}
```

#### 情况 3：主动清理
```rust
// cleanup_expired() 方法会清理所有过期数据
// 但这个方法需要手动调用，不会自动执行
pub async fn cleanup_expired(&self) -> Result<usize> {
    // 清理所有过期项（包括 stale 数据）
}
```

**清除逻辑总结：**
- ❌ **不会自动清理**：过期数据不会自动删除
- ✅ **访问时检查**：每次 `get()` 时检查是否过期
- ✅ **超过 max_stale 才删除**：如果启用 SWR，只有在超过 `max_stale_seconds` 时才删除
- ✅ **可以手动清理**：调用 `cleanup_expired()` 可以清理所有过期数据

### 3. 直接获取缓存，如果有数据即使过期也直接使用？

**答案：是的，但需要满足条件。**

**条件：**
1. ✅ 启用 SWR：`config.swr.enabled = true`
2. ✅ 数据过期：`metadata.is_expired() == true`
3. ✅ 在 max_stale 范围内：`stale_age <= max_stale_seconds`

**工作流程：**
```rust
pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
    // 1. 获取元数据
    let metadata = index.get(key);
    
    // 2. 检查是否过期
    if metadata.is_expired() {
        // 3. 检查是否可以返回 stale 数据
        if swr_enabled && metadata.is_stale_acceptable(max_stale_seconds) {
            // ✅ 返回过期数据（stale data）
            return Ok(Some(stale_data));
        } else {
            // ❌ 删除并返回 None
            invalidate(key);
            return Ok(None);
        }
    }
    
    // 4. 未过期，返回新鲜数据
    return Ok(Some(fresh_data));
}
```

## 实际示例

### 示例 1：正常过期（禁用 SWR）

```rust
// 配置
config.swr.enabled = false;
config.default_ttl_seconds = 5;

// 时间线
0s  - set("key", "value")  // 设置缓存，TTL=5s
5s  - 缓存过期
6s  - get("key")           // ❌ 返回 None，数据已删除
```

### 示例 2：SWR 返回 stale 数据

```rust
// 配置
config.swr.enabled = true;
config.swr.max_stale_seconds = 3600;  // 1小时
config.default_ttl_seconds = 5;

// 时间线
0s  - set("key", "value")  // 设置缓存，TTL=5s
5s  - 缓存过期
6s  - get("key")           // ✅ 返回 "value" (stale data)
     - stale_age = 1s < 3600s，可以返回
10s - get("key")           // ✅ 返回 "value" (stale data)
     - stale_age = 5s < 3600s，可以返回
```

### 示例 3：超过 max_stale 后删除

```rust
// 配置
config.swr.enabled = true;
config.swr.max_stale_seconds = 10;  // 10秒
config.default_ttl_seconds = 5;

// 时间线
0s  - set("key", "value")  // 设置缓存，TTL=5s
5s  - 缓存过期
6s  - get("key")           // ✅ 返回 "value" (stale_age=1s < 10s)
15s - get("key")           // ❌ 返回 None，数据已删除
     - stale_age = 10s >= 10s，超过 max_stale，删除数据
```

## 内存和存储影响

### 内存使用
- ✅ **不增加内存**：只保存一份数据
- ✅ **元数据开销小**：只增加过期时间检查

### 存储使用
- ✅ **不增加存储**：文件系统中只有一份数据文件
- ⚠️ **可能占用空间**：过期数据会继续占用空间，直到被清理

### 清理建议
```rust
// 定期清理过期数据（包括 stale 数据）
cache.cleanup_expired().await?;

// 或者在应用启动时清理
cache.load_index().await?;  // 加载时会清理过期文件
```

## 总结

| 问题 | 答案 |
|------|------|
| 保存两份数据？ | ❌ 否，只保存一份 |
| 过期立即清除？ | ❌ 否，有条件的延迟清除 |
| 过期数据可用？ | ✅ 是，如果启用 SWR 且在 max_stale 范围内 |
| 自动清理？ | ❌ 否，需要手动调用 `cleanup_expired()` |
| 访问时检查？ | ✅ 是，每次 `get()` 都会检查 |

## 最佳实践

1. **设置合理的 max_stale_seconds**：不要设置太大，避免占用过多空间
2. **定期清理**：在应用启动或定期任务中调用 `cleanup_expired()`
3. **监控 stale 数据**：使用 `is_stale()` 方法检查数据状态
4. **后台更新**：当返回 stale 数据时，在后台触发数据更新

