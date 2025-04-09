import { API_URI } from '@/constants';
import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';

export function createFetcher<TRequest, TResponse> (action: string, method?: string) {
  const url = `${API_URI}/v1${action}`;

  return async function(data?: TRequest): Promise<TResponse> {
    const cookieStore = await cookies();
    const token = cookieStore.get(SESSION_ID)?.value || '';

    try {
      const res = await fetch(url, {
        method: method || 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify(data),
      });
      const json = await res.json() as any;
  
      if (json?.code === '200') {
        return json?.data as TResponse;
      } else {
        console.log('FETCHER:', json);
        return json;
      }
    } catch (err) {
      console.log('fetcher ERR]', err);
      return err as any as TResponse;
    }
  }
}