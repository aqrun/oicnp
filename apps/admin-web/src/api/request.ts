import type { AxiosRequestConfig, Method } from 'axios';

import { message as $message } from 'antd';
import axios from 'axios';
import { useAppStore } from '~/stores';

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
    if (method?.toLowerCase() === 'post') {
      const res = axiosInstance.post<TRequest, TResponse>(url, data, config);
      return res;
    } else {
      const res = axiosInstance.get<TRequest, TResponse>(url, {
        params: data,
        ...config,
      });
      return res;
    }
  }
}
