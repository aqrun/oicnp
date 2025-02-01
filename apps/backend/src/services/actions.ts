import { createFetcher } from './fetcher';
import {
  DescribeMenuListRequestParams,
  DescribeMenuTreeResponseData,
} from './types';

/**
 * 获取菜单树
 */
export const DescribeMenuTree = createFetcher<
DescribeMenuListRequestParams,
DescribeMenuTreeResponseData
>('/menu/tree', 'post');
