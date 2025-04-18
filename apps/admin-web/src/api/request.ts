import type { AxiosRequestConfig, Method } from 'axios';

import { message as $message } from 'antd';
import axios from 'axios';
import { useAppStore } from '~/stores';

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
  config => {
    useAppStore.setState({
      loading: false,
    });

    if (config?.data?.message) {
      $message.success(config.data.message)
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
    // error.message && $message.error(errorMessage);

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
  uri: string,
  method: Method,
  config: AxiosRequestConfig = {}
): (data?: TRequest) => Promise<TResponse> {
  const url = `/api${uri}`;

  return (data?: TRequest) => {
    return new Promise<TResponse>((resolve) => {
      if (method?.toLowerCase() === 'post') {
        const newConfig: AxiosRequestConfig = {
          ...config,
          params: {
            ...(config?.params || {}),
            // 添加 url 参数 方便控制台调试查看
            _fetcher: uri?.replace('/', '').replace(/\//i, '-'),
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
