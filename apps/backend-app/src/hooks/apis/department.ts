import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { departmentApis } from '#src/api';
import type { DescribeDepartmentListRequestParams } from '@repo/apis';

/**
 * 获取部门列表
 */
export function useFetchDepartmentList() {
	const [loading, setLoading] = useState(false);

	const fetchDepartmentList = useMemoizedFn(async (params: DescribeDepartmentListRequestParams) => {
		setLoading(true);
		const res = await departmentApis.DescribeDepartmentList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchDepartmentList,
	};
}
