import type { NextConfig } from "next";
import { getApiUri } from '@repo/services/url';

const nextConfig: NextConfig = {
  transpilePackages: ['@repo/services'],
  sassOptions: {
    implementation: 'sass-embedded',
  },
  compiler: {
    styledComponents: true,
  },
  // API 转发配置 - 仅在开发环境使用 rewrites
  // 生产环境建议使用 nginx 反向代理，性能更好
  async rewrites() {
    // 只在开发环境启用 rewrites，生产环境使用 nginx
    if (process.env.NODE_ENV === 'development') {
      return [
        {
          source: '/api/:path*',
          destination: `${getApiUri()}/:path*`, // 例如: /api/v1/xxx -> http://localhost:5150/v1/xxx
        },
      ];
    }
    return [];
  },
  images: {
    remotePatterns: [
      {
        protocol: 'http',
        hostname: 'localhost',
        port: '',
      },
    ],
  },
};

export default nextConfig;
