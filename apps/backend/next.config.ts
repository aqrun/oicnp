import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  sassOptions: {
    implementation: 'sass-embedded',
  },
  compiler: {
    styledComponents: true,
  },
};

export default nextConfig;
