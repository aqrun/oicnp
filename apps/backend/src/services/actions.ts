import { createFetcher } from './fetcher';
import {
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
} from './types';

/**
 * 获取菜单树
 */
export const DescribeAuthLogin = createFetcher<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');
