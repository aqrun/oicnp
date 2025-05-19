import { createFetcher } from './fetcher';
import {
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
} from './types';

/**
 * 登录
 */
export const DescribeAuthLogin = createFetcher<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');
