import { createFetcher } from './fetcher';
import {
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
  DescribeAuthInfoRequestParams,
  DescribeAuthInfoResponseData,
} from './types';

/**
 * 登录
 */
export const DescribeAuthLogin = createFetcher<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');

/**
 * 获取当前登录用户信息
 */
export const DescribeAuthInfo = createFetcher<
DescribeAuthInfoRequestParams,
DescribeAuthInfoResponseData
>('/auth/info', 'post');
