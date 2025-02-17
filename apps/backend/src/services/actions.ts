import { createFetcher } from './fetcher';
import {
  DescribeMenuListRequestParams,
  DescribeMenuTreeResponseData,
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
  DescribeUserRequestParams,
  DescribeUserResponseData,
} from './types';

/**
 * 获取菜单树
 */
export const DescribeMenuTree = createFetcher<
DescribeMenuListRequestParams,
DescribeMenuTreeResponseData
>('/menu/tree', 'post');

/**
 * 获取菜单树
 */
export const DescribeAuthLogin = createFetcher<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');

export const DescribeUser = createFetcher<
DescribeUserRequestParams,
DescribeUserResponseData
>('/user/one', 'post');
