import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { roleApis } from '#src/api';
import type { DescribeRoleListRequestParams } from '@repo/apis';

/**
 * 获取角色列表
 */
export function useFetchRoleList() {
	const [loading, setLoading] = useState(false);

	const fetchRoleList = useMemoizedFn(async (params: DescribeRoleListRequestParams = {}) => {
		setLoading(true);
		const res = await roleApis.DescribeRoleList(params);
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchRoleList,
	};
}
