import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { cronApis } from '#src/api';
import type { DescribeCronListRequestParams } from '@repo/apis';

/**
 * 获取定时任务列表
 */
export function useFetchCronList() {
	const [loading, setLoading] = useState(false);

	const fetchCronList = useMemoizedFn(async (params: DescribeCronListRequestParams = {}) => {
		setLoading(true);
		const res = await cronApis.DescribeCronList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchCronList,
	};
}
