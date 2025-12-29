#!/bin/bash
# sync.sh - 同步 .next 目录到服务器
# 使用方法: ./sync.sh [服务器地址] [用户名] [目标路径]
# 示例: ./sync.sh 192.168.1.100 root /path/to/project
# rm -rf apps/web/.next && unzip -q -o target/web.zip -d apps/web/
# rm -rf apps/backend/.next && unzip -q -o target/backend.zip -d apps/backend/

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置变量（可以通过参数或环境变量覆盖）
SERVER_HOST="${1:-${SERVER_HOST:-}}"
SERVER_USER="${2:-${SERVER_USER:-root}}"
REMOTE_PATH="${3:-${REMOTE_PATH:-}}"

# 本地路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WEB_DIR="$PROJECT_ROOT/apps/web"
WEB_NEXT_DIR="$WEB_DIR/.next"
WEB_ZIP_FILE="$PROJECT_ROOT/target/web.zip"

BACKEND_DIR="$PROJECT_ROOT/apps/backend"
BACKEND_NEXT_DIR="$BACKEND_DIR/.next"
BACKEND_ZIP_FILE="$PROJECT_ROOT/target/backend.zip"

# 压缩 web/.next 目录
echo -e "${GREEN}📦 压缩 web/.next 目录...${NC}"
rm -f "$WEB_ZIP_FILE"
cd "$WEB_DIR"
zip -r "$WEB_ZIP_FILE" .next/ > /dev/null 2>&1
WEB_ZIP_SIZE=$(du -h "$WEB_ZIP_FILE" | cut -f1)
echo -e "${GREEN}✅ web 压缩完成: $WEB_ZIP_FILE (大小: $WEB_ZIP_SIZE)${NC}"

# 压缩 backend/.next 目录
echo -e "${GREEN}📦 压缩 backend/.next 目录...${NC}"
rm -f "$BACKEND_ZIP_FILE"
cd "$BACKEND_DIR"
zip -r "$BACKEND_ZIP_FILE" .next/ > /dev/null 2>&1
BACKEND_ZIP_SIZE=$(du -h "$BACKEND_ZIP_FILE" | cut -f1)
echo -e "${GREEN}✅ backend 压缩完成: $BACKEND_ZIP_FILE (大小: $BACKEND_ZIP_SIZE)${NC}"

echo -e "${GREEN}🎉 压缩完成！${NC}"


