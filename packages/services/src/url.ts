export function getApiUri() {
  const apiUri = process.env.URL_API || 'http://localhost:5150';
  return apiUri;
}