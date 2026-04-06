import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { positionApis } from '#src/api';
import type { DescribePositionListRequestParams } from '@repo/apis';

/**
 * 获取职位列表
 */
export function useFetchPositionList() {
	const [loading, setLoading] = useState(false);

	const fetchPositionList = useMemoizedFn(async (params: DescribePositionListRequestParams) => {
		setLoading(true);
		const res = await positionApis.DescribePositionList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchPositionList,
	};
}
