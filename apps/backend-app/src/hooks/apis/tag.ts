import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { tagApis } from '#src/api';
import type { DescribeTagListRequestParams } from '@repo/apis';

/**
 * 获取标签列表
 */
export function useFetchTagList() {
	const [loading, setLoading] = useState(false);

	const fetchTagList = useMemoizedFn(async (params: DescribeTagListRequestParams) => {
		setLoading(true);
		const res = await tagApis.DescribeTagList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchTagList,
	};
}
