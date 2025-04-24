import { useMemoizedFn } from "ahooks";
import {
  DescribeUserRolesRequestParams,
  DescribeUserDetailRequestParams,
} from './types';
import {
  DescribeUserRoles,
  DescribeUserDetail,
} from "./client";

/**
 * 获取用户角色列表
 */
export function useFetchUserRoles() {
  const fetchUserRoles = useMemoizedFn(async (params: DescribeUserRolesRequestParams = {}) => {
    const res = await DescribeUserRoles(params);
    return res;
  });

  return {
    fetchUserRoles,
  };
}

/**
 * 获取用户详细信息
 */
export function useFetchUser() {
  const fetchUser = useMemoizedFn(async (params: DescribeUserDetailRequestParams = {}) => {
    const res = await DescribeUserDetail(params);
    return res;
  });

  return {
    fetchUser,
  };
}
