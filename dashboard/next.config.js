/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  env: {
    BOT_API_URL: process.env.BOT_API_URL || 'http://localhost:8080',
  },
}

module.exports = nextConfig
