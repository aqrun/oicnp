import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { fileApis } from '#src/api';
import type {
	DescribeFileListRequestParams,
	DescribeFileDetailRequestParams,
} from '@repo/apis';

/**
 * 获取文件列表
 */
export function useFetchFileList() {
	const [loading, setLoading] = useState(false);

	const fetchFileList = useMemoizedFn(async (params: DescribeFileListRequestParams = {}) => {
		setLoading(true);
		const res = await fileApis.DescribeFileList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchFileList,
	};
}

export function useFetchFile() {
	const [loading, setLoading] = useState(false);

	const fetchFile = useMemoizedFn(async (params: DescribeFileDetailRequestParams = {}) => {
		setLoading(true);
		const res = await fileApis.DescribeFileDetail(params);
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchFile,
	};
}
