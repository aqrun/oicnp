import { API_URI } from '@/constants';

export function createFetcher<TRequest, TResponse> (action: string, method?: string) {
  const url = `${API_URI}/v1${action}`;
  return (data?: TRequest) => {
    return new Promise<TResponse>((resolve) => {
      fetch(url, {
        method: method || 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
      }).then((res) => res.json()).then((res) => {
        resolve(res?.data);
      }).catch(err => {
        resolve(null as any);
      });
    });
  }
}