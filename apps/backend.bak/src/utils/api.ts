import axios from 'axios';
import {
  QueryClient,
} from '@tanstack/react-query';
import { BASE_ADMIN_API_URL } from '~/constants';

export const queryClient = new QueryClient();

interface AxiosResponse {
  data?: {
    code?: string;
    message?: string;
    data?: unknown;
  }
}

export function api<
TResponse extends NonNullable<unknown>,
TRequest extends NonNullable<unknown>
>(
  action: string,
  method = 'post'
): (params: TRequest) => Promise<TResponse> {
  const url = `${BASE_ADMIN_API_URL}/${action}`;

  return (params: TRequest): Promise<TResponse> => {
    if (method === 'post') {
      return new Promise<TResponse>((resolve) => {
        axios.post<TRequest, TResponse>(url, params).then((res) => {
          const _res = res as unknown as AxiosResponse;
          const code = _res.data?.code;

          // 返回成功
          if (!code || code === '200') {
            resolve(_res.data?.data as TResponse);
          }
        }).catch(err => {
          // eslint-disable-next-line no-console -- some error
          console.log('ERR]', err)
          resolve(null as unknown as TResponse);
        });
      });
    }

    const res = new Promise<TResponse>((resolve) => {
      resolve({} as TResponse)
    });
    return res;
  }
}
