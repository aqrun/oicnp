import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { menuApis } from '#src/api';
import type {
	DescribeMenuListRequestParams,
	DescribeMenuTreeRequestParams,
} from '@repo/apis';

/**
 * 获取菜单列表
 */
export function useFetchMenuList() {
	const [loading, setLoading] = useState(false);

	const fetchMenuList = useMemoizedFn(async (params: DescribeMenuListRequestParams = {}) => {
		setLoading(true);
		const res = await menuApis.DescribeMenuList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchMenuList,
	};
}

/**
 * 获取菜单树
 */
export function useFetchMenuTree() {
	const [loading, setLoading] = useState(false);

	const fetchMenuTree = useMemoizedFn(async (params: DescribeMenuTreeRequestParams = {}) => {
		setLoading(true);
		const res = await menuApis.DescribeMenuTree(params);
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchMenuTree,
	};
}
