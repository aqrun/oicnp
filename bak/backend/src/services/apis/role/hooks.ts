import {
  DescribeRoleList,
} from './client';
import {
  DescribeRoleListRequestParams,
} from './types';
import { useMemoizedFn } from 'ahooks';

export function useFetchRoleList() {
  const fetchRoleList = useMemoizedFn(async (params: DescribeRoleListRequestParams = {}) => {
    const res = await DescribeRoleList(params);
    return res;
  });

  return {
    fetchRoleList,
  };
}
