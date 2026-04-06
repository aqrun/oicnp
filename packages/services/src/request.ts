import type { Options as KyOptions } from 'ky';
import ky from 'ky';
import type { BaseResponse } from './types';

export type Options = KyOptions & {
	ignoreError?: boolean;
};

export const defaultConfig: Options = {
	// The input argument cannot start with a slash / when using prefixUrl option.
	prefixUrl: '/api',
	credentials: "include",
	timeout: 10000,
	retry: {
		// 当请求失败时，最多重试次数
		limit: 3,
	},
};

export function createRequest(config: Options = defaultConfig) {
  const request = ky.create({
    ...defaultConfig,
    ...config,
  });
  return request;
}


export interface RequestConfig {
  ignoreError?: boolean;
}

export function createServiceFactory(
  request: ReturnType<typeof ky.create>,
) {
  function createService<TRequest, TResponse extends BaseResponse> (
    action: string,
    method: string,
    options: Options = {}
  ): (data?: TRequest) => Promise<TResponse> {
    return (data?: TRequest) => {
      return new Promise<TResponse>((resolve, reject) => {
        if (method?.toLowerCase() === 'post') {
          request.post(action, { json: data, ...options })
            .json<TResponse>()
            .then(res => resolve(res?.data as TResponse))
            .catch(err => reject(err));
        } else {
          request.get(action, { searchParams: data || {}, ...options })
            .json<TResponse>()
            .then(res => resolve(res?.data as TResponse))
            .catch(err => reject(err));
        }
      });
    };
  }

  return createService;
}

export type CreateService = ReturnType<typeof createServiceFactory>;