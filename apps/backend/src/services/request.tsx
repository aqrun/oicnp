'use client';

import type { AxiosRequestConfig, Method } from 'axios';
import axios from 'axios';
import { useAppStore } from '@/stores';
import { API_URI } from '@/constants';

export type RequestConfig = AxiosRequestConfig & {
  ignoreError?: boolean;
};

/**
 * Response 公共数据
 */
export interface BaseResponseData<TResponse> {
  code: string;
  data: TResponse,
  message: string;
}

const axiosInstance = axios.create({
  timeout: 6000,
});

axiosInstance.interceptors.request.use(
  config => {
    useAppStore.setState({
      loading: true,
    });

    return config;
  },
  error => {
    useAppStore.setState({
      loading: false,
    });
    Promise.reject(error);
  },
);

axiosInstance.interceptors.response.use(
  (config) => {
    useAppStore.setState({
      loading: false,
    });

    const ignoreError = (config?.config as RequestConfig)?.ignoreError ?? false;
    const code = config?.data?.code ?? '200';
    const message = config?.data?.message;

    if (!ignoreError && code !== '200' && message) {
      console.error(message);
      useAppStore.setState({
        loading: false,
        errors: [{
          code,
          message,
        }]
      });
    }

    return config?.data;
  },
  error => {
    useAppStore.setState({
      loading: false,
    });
    // if needs to navigate to login page when request exception
    // history.replace('/login');
    let errorMessage = '系统异常';

    if (error?.message?.includes('Network Error')) {
      errorMessage = '网络错误，请检查您的网络';
    } else {
      errorMessage = error?.message;
    }

    console.dir(error);
    if (error.message) {
      useAppStore.setState({
        loading: false,
        errors: [{
          code: '500',
          message: errorMessage,
        }]
      });
    }

    return {
      status: false,
      message: errorMessage,
      result: null,
    };
  },
);

/**
 * 创建服务
 */
export function createService<TRequest, TResponse> (
  action: string,
  method: Method,
  config: RequestConfig = {}
): (data?: TRequest) => Promise<TResponse> {
  const url = `${API_URI}/v1${action}`;

  return (data?: TRequest) => {
    return new Promise<TResponse>((resolve) => {
      if (method?.toLowerCase() === 'post') {
        const newConfig: RequestConfig = {
          ...config,
          params: {
            ...(config?.params || {}),
            // 添加 url 参数 方便控制台调试查看
            _fetcher: action?.replace('/', '').replace(/\//i, '-'),
          },
        };

        axiosInstance.post<TRequest, BaseResponseData<TResponse>>(url, data, newConfig).then((res) => {
          resolve(res?.data);
        });
      } else {
        axiosInstance.get<TRequest, BaseResponseData<TResponse>>(url, {
          params: data,
          ...config,
        }).then((res) => {
          resolve(res?.data);
        });
      }
    });
  };
}
