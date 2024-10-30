import axios from 'axios';

export function api<
TResponse extends NonNullable<unknown>,
TRequest extends NonNullable<unknown>
>(
  action: string,
  method = 'post'
): (params: TRequest) => Promise<TResponse> {
  const url = action;

  return (params: TRequest): Promise<TResponse> => {
    if (method === 'post') {
      const res = axios.post<TRequest, TResponse>(url, params);
      return res;
    }

    const res = new Promise<TResponse>((resolve) => {
      resolve({} as TResponse)
    });
    return res;
  }
}
