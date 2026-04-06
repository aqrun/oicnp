import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { onlineApis } from '#src/api';
import type { DescribeOnlineListRequestParams } from '@repo/apis';

/**
 * 获取在线用户列表
 */
export function useFetchOnlineList() {
	const [loading, setLoading] = useState(false);

	const fetchOnlineList = useMemoizedFn(async (params: DescribeOnlineListRequestParams = {}) => {
		setLoading(true);
		const res = await onlineApis.DescribeOnlineList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchOnlineList,
	};
}
