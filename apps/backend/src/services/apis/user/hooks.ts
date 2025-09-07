'use client';

import { useState } from "react";
import { useMemoizedFn } from "ahooks";
import {
  DescribeUserListRequestParams,
  DescribeUserRolesRequestParams,
  DescribeUserDetailRequestParams,
} from './types';
import { useAppStore } from '@/stores/useAppStore';
import {
  DescribeUserRoles,
  DescribeUserDetail,
  DescribeUserList,
} from "./client";
import { DescribeAuthInfo } from './client';

/**
 * 获取用户列表
 */
export function useFetchUserList() {
  const [loading, setLoading] = useState(false);

  const fetchUserList = useMemoizedFn(async (params: DescribeUserListRequestParams) => {
    setLoading(true);
    const res = await DescribeUserList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchUserList,
  };
}


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

/**
 * 获取当前登陆用户信息
 */
export function useGetCurrentUser() {
  const stateUser = useAppStore(state => state.user);
  const setAppState = useAppStore(state => state.setState);

  const getCurrentUser = useMemoizedFn(async (refresh = false) => {
    let user = stateUser;

    // 获取 cookie 数据
    const userData = await getUserData();
    
    if (!userData || !userData?.uuid) {
      setAppState({
        errors: [{
          code: 'UserNeedLogin',
          message: '用户未登录',
        }],
      });
      return undefined;
    }

    if (refresh) {
      const userRes = await DescribeUserDetail({
        uuid: userData?.uuid,
      });

      if (userRes?.code && userRes?.code !== '200') {
        return undefined;
      }

      setAppState({
        user: userRes?.user,
      });
      user = userRes?.user;
    }

    return user;
  });

  return {
    getCurrentUser,
  };
}

/**
 * 根据cookie获取用户登陆信息
 */
export async function getUserData() {
  const res = await DescribeAuthInfo({});
  return res?.user;
}
