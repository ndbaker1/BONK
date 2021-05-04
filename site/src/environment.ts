const SSL = !!process.env.NEXT_PUBLIC_SSL
export const environment = {
  apiDomain: process.env.NEXT_PUBLIC_API_DOMAIN || 'localhost:8000',
  ws_or_wss: SSL ? 'wss' : 'ws',
  http_or_https: SSL ? 'https' : 'http',
  healthCheck: true,
}