import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { userApis } from '#src/api';
import type {
	DescribeUserListRequestParams,
	DescribeUserRolesRequestParams,
	DescribeUserDetailRequestParams,
} from '@repo/apis';

/**
 * 获取用户列表
 */
export function useFetchUserList() {
	const [loading, setLoading] = useState(false);

	const fetchUserList = useMemoizedFn(async (params: DescribeUserListRequestParams) => {
		setLoading(true);
		const res = await userApis.DescribeUserList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);

		return res;
	});

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
		const res = await userApis.DescribeUserRoles(params);
		return res;
	});

	return {
		fetchUserRoles,
	};
}

/**
 * 获取用户详情
 */
export function useFetchUser() {
	const fetchUser = useMemoizedFn(async (params: DescribeUserDetailRequestParams = {}) => {
		const res = await userApis.DescribeUserDetail(params);
		return res;
	});

	return {
		fetchUser,
	};
}

/**
 * 根据会话信息获取当前用户（auth/info）
 */
export async function getUserData() {
	const res = await userApis.DescribeAuthInfo({});
	return res?.user;
}
