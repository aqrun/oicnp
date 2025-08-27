import { API_URI } from '@/constants';
import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';
import { BaseResponse } from './types';

/**
 * server端接口创建
 */
export function createFetcher<TRequest, TResponse extends BaseResponse> (action: string, method?: string) {
  const url = `${API_URI}/v1${action}`;

  return async function(data?: TRequest): Promise<TResponse> {
    const cookieStore = await cookies();
    const token = cookieStore.get(SESSION_ID)?.value || '';
    
    let json = {} as TResponse;

    try {
      const res = await fetch(url, {
        method: method || 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify(data || {}),
      });
  
      json = await res.json() as unknown as TResponse;
    } catch (err) {
      json = {
        code: '500',
        data: null,
        message: (err as Error).toString(),
      } as unknown as TResponse;
    }

    if (json?.code === '200') {
      return json?.data as TResponse;
    } else {
      console.log('FETCHER:', json);
      return json;
    }
  }
}