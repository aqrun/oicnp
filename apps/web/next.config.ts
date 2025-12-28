import { getApiUri } from '@repo/services/url';

/** @type {import('next').NextConfig} */
const nextConfig = {
  // output: 'export',
  eslint: {
    dirs: ['src'],
  },

  transpilePackages: ['@repo/apis', '@repo/services'],

  // reactStrictMode: true,
  // swcMinify: true,
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

  // Uncoment to add domain whitelist
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'cdn.oicnp.com',
        port: '',
        pathname: '/**',
      },
    ],
  },

  webpack(config: any, { isServer }: { isServer: boolean }) {
    // Ignore Node.js built-in modules in client-side bundles
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        path: false,
        module: false,
        crypto: false,
        os: false,
        tty: false,
        worker_threads: false,
        'node:fs': false,
        'node:fs/promises': false,
        'node:path': false,
        'node:module': false,
        'node:crypto': false,
        'node:os': false,
        'node:tty': false,
        'node:worker_threads': false,
      };
    }

    // Grab the existing rule that handles SVG imports
    const fileLoaderRule = config.module.rules.find((rule: any) =>
      rule.test?.test?.('.svg')
    );

    config.module.rules.push(
      // Reapply the existing rule, but only for svg imports ending in ?url
      {
        ...fileLoaderRule,
        test: /\.svg$/i,
        resourceQuery: /url/, // *.svg?url
      },
      // Convert all other *.svg imports to React components
      {
        test: /\.svg$/i,
        issuer: { not: /\.(css|scss|sass)$/ },
        resourceQuery: { not: /url/ }, // exclude if *.svg?url
        loader: '@svgr/webpack',
        options: {
          dimensions: false,
          titleProp: true,
        },
      }
    );

    // Modify the file loader rule to ignore *.svg, since we have it handled now.
    fileLoaderRule.exclude = /\.svg$/i;

    return config;
  },
};

module.exports = nextConfig;
