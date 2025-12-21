export function getApiUri() {
  const apiUri = process.env.NEXT_PUBLIC_OICNP_API_URI || 'http://localhost:5150';
  return apiUri;
}