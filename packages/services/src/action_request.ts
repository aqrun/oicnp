import { BaseResponse } from './types';
import { getApiUri } from './url';

export function createActionService<TRequest, TResponse extends BaseResponse> (action: string, method?: string) {
  const url = `${getApiUri()}/v1${action}`;

  return async function(data?: TRequest): Promise<TResponse> {
    let json = {} as TResponse;

    try {
      const res = await fetch(url, {
        method: method || 'POST',
        headers: {
          'Content-Type': 'application/json',
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
