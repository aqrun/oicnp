# 测试说明

## 运行测试

### 运行所有测试
```bash
cargo test --package oic_cache
```

### 运行集成测试
```bash
cargo test --package oic_cache --test integration_test
```

### 运行并发测试
```bash
cargo test --package oic_cache --test concurrent_test
```

### 如果测试显示"没有更新"或"no tests to run"

可能的原因和解决方案：

1. **清理构建缓存**
   ```bash
   cargo clean -p oic_cache
   cargo test --package oic_cache --test integration_test
   ```

2. **强制重新编译**
   ```bash
   cargo test --package oic_cache --test integration_test --no-fail-fast
   ```

3. **检查编译错误**
   ```bash
   cargo check --package oic_cache --tests
   ```

4. **详细输出模式**
   ```bash
   cargo test --package oic_cache --test integration_test -- --nocapture --test-threads=1
   ```

5. **从项目根目录运行**
   ```bash
   cd /path/to/oicnp
   cargo test --package oic_cache --test integration_test
   ```

6. **检查测试文件是否存在**
   ```bash
   ls -la crates/oic-cache/tests/
   ```

## 测试文件结构

- `integration_test.rs` - 集成测试（13个测试用例）
- `concurrent_test.rs` - 并发测试（4个测试用例）

## 测试用例说明

### integration_test.rs
- `test_basic_get_set` - 基础读写测试
- `test_expiration` - 过期时间测试
- `test_inline_storage` - 内联存储测试（<4KB）
- `test_file_storage` - 文件存储测试（>=4KB）
- `test_invalidate` - 删除测试
- `test_namespace_invalidation` - 命名空间失效测试
- `test_tag_invalidation` - 标签失效测试
- `test_vary_cache` - Vary 缓存测试
- `test_stats_tracking` - 统计追踪测试
- `test_batch_get` - 批量获取测试
- `test_cleanup_expired` - 过期清理测试
- `test_index_persistence` - 索引持久化测试
- `test_clear` - 清空测试

### concurrent_test.rs
- `test_concurrent_reads` - 并发读取测试（1000个并发）
- `test_concurrent_writes` - 并发写入测试（100个并发）
- `test_concurrent_mixed` - 混合并发测试（读写混合）
- `test_concurrent_different_keys` - 不同键并发写入测试

