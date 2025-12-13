#!/bin/bash
# docker-build-tsinghua.sh
  
set -e

echo "🚀 使用 Docker 编译（清华镜像）..."

CMD=$(cat <<'EOF'
set -e
# 备份并完全替换 sources.list
cp /app/scrips/sources.list /etc/apt/sources.list
# 删除 sources.list.d 目录下的所有配置文件（这些可能包含原地址）
rm -f /etc/apt/sources.list.d/*.list 2>/dev/null || true
rm -f /etc/apt/sources.list.d/*.sources 2>/dev/null || true
# 确保 .cargo 目录存在
mkdir -p /root/.cargo
# 复制 cargo 配置文件
cp /app/scrips/config.toml /root/.cargo/config.toml
# 彻底清理 cargo 所有缓存，强制使用新配置
rm -rf /root/.cargo/registry/index/* 2>/dev/null || true
rm -rf /root/.cargo/registry/cache/* 2>/dev/null || true
rm -rf /root/.cargo/.package-cache 2>/dev/null || true
rm -rf /root/.cargo/registry/src/* 2>/dev/null || true
# 验证源配置
echo '📋 当前使用的 apt 源配置:'
grep -v '^#' /etc/apt/sources.list | grep -v '^$' | head -3
echo ''
echo '📋 当前使用的 cargo 源配置:'
cat /root/.cargo/config.toml
echo ''
echo '📥 更新包列表（仅使用清华镜像）...'
apt-get update -qq > /dev/null 2>&1
echo '📦 安装依赖包...'
apt-get install -y musl-tools libssl-dev pkg-config
echo '🔧 添加 musl 目标...'
rustup target add x86_64-unknown-linux-musl
echo '🧪 验证 cargo 配置...'
echo "Cargo 版本: $(cargo --version)"
echo "实际配置位置: /root/.cargo/config.toml"
echo "配置内容验证:"
grep -A1 'replace-with' /root/.cargo/config.toml || echo "警告：未找到 replace-with 配置"
echo ''
echo '🔨 开始编译（使用 rsproxy.cn 镜像）...'
echo 'ℹ️  提示：Cargo 可能仍显示 "Updating crates.io index"，但配置已指向 rsproxy.cn'
echo 'ℹ️  可通过网络监控工具验证实际访问的地址'
OPENSSL_STATIC=1 CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --target x86_64-unknown-linux-musl --release
echo '✅ 编译完成！'
EOF
)

sudo docker run --rm \
  -v "$(pwd)":/app \
  -e RUSTUP_DIST_SERVER="https://rsproxy.cn" \
  -e RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
  -w /app \
  rust:latest \
  bash -c "$CMD"

echo "📁 编译完成！"