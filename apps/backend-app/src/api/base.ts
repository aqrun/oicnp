import { request } from "#src/utils/request";

export interface RequestConfig {
  ignoreError?: boolean;
}

export function createService<TRequest, TResponse> (
  action: string,
  method: string,
  config: RequestConfig = {}
): (data?: TRequest) => Promise<TResponse> {
  return (data?: TRequest) => {
    return new Promise<TResponse>((resolve, reject) => {
      if (method?.toLowerCase() === 'post') {
        request.post(action, { json: data })
          .json<TResponse>()
          .then(res => resolve(res))
          .catch(err => reject(err));
      } else {
        request.get("get-async-routes")
          .json<TResponse>()
          .then(res => resolve(res))
          .catch(err => reject(err));
      }
    });
  };
}