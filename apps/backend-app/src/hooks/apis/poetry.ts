import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { poetryApis } from '#src/api';
import type { DescribePoetryListWithChaptersRequestParams } from '@repo/apis';

/**
 * 获取诗词列表（含章节）
 */
export function useFetchPoetryListWithChapters() {
	const [loading, setLoading] = useState(false);

	const fetchPoetryListWithChapters = useMemoizedFn(
		async (params: DescribePoetryListWithChaptersRequestParams = {}) => {
			setLoading(true);
			const res = await poetryApis.DescribePoetryListWithChapters({
				page: params.page || 1,
				pageSize: params.pageSize || 10,
				...(params || {}),
			});
			setLoading(false);
			return res;
		},
	);

	return {
		loading,
		fetchPoetryListWithChapters,
	};
}
