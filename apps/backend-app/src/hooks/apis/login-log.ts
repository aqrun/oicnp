import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { loginLogApis } from '#src/api';
import type { DescribeLoginLogListRequestParams } from '@repo/apis';

/**
 * 获取登录日志列表
 */
export function useFetchLoginLogList() {
	const [loading, setLoading] = useState(false);

	const fetchLoginLogList = useMemoizedFn(async (params: DescribeLoginLogListRequestParams) => {
		setLoading(true);
		const res = await loginLogApis.DescribeLoginLogList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchLoginLogList,
	};
}
