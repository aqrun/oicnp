import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { permissionApis } from '#src/api';
import type {
	DescribePermissionListRequestParams,
	DescribePermissionTreeRequestParams,
} from '@repo/apis';

/**
 * 获取权限列表
 */
export function useFetchPermissionList() {
	const [loading, setLoading] = useState(false);

	const fetchPermissionList = useMemoizedFn(async (params: DescribePermissionListRequestParams = {}) => {
		setLoading(true);
		const res = await permissionApis.DescribePermissionList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchPermissionList,
	};
}

/**
 * 获取权限树
 */
export function useFetchPermissionTree() {
	const [loading, setLoading] = useState(false);

	const fetchPermissionTree = useMemoizedFn(async (params: DescribePermissionTreeRequestParams = {}) => {
		setLoading(true);
		const res = await permissionApis.DescribePermissionTree(params);
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchPermissionTree,
	};
}
