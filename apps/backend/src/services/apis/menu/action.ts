import { createFetcher } from '../../fetcher';
import {
  DescribeMenuTreeRequestParams,
  DescribeMenuTreeResponseData,
} from './types';

/**
 * 获取菜单树
 */
export const DescribeMenuTree = createFetcher<
DescribeMenuTreeRequestParams,
DescribeMenuTreeResponseData
>('/menu/tree', 'post');