/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack: (config, context) => {
    config.experiments = {
      ...config.experiments,
      topLevelAwait: true,
      asyncWebAssembly: true,
    };

    config.module.rules.push({
      test: /\.md$/i,
      use: "raw-loader",
    });

    return config;
  },
  transpilePackages: ["@visheratin/web-ai"],
};

module.exports = nextConfig;
