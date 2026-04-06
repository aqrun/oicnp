import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { operationLogApis } from '#src/api';
import type { DescribeOperationLogListRequestParams } from '@repo/apis';

/**
 * 获取操作日志列表
 */
export function useFetchOperationLogList() {
	const [loading, setLoading] = useState(false);

	const fetchOperationLogList = useMemoizedFn(async (params: DescribeOperationLogListRequestParams) => {
		setLoading(true);
		const res = await operationLogApis.DescribeOperationLogList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchOperationLogList,
	};
}
