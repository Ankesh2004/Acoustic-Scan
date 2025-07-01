/** @type {import('next').NextConfig} */
const nextConfig = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  images: {
    unoptimized: true,
  },
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination:
          process.env.NODE_ENV === "production"
            ? "https://acoustic-scan.onrender.com/api/:path*"
            : "http://localhost:8080/api/:path*", // Proxy to Backend
      },
    ];
  },
};

export default nextConfig;
