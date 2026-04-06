import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { noteApis } from '#src/api';
import type {
	DescribeNoteListRequestParams,
	DescribeNoteDetailRequestParams,
} from '@repo/apis';

/**
 * 获取笔记列表
 */
export function useFetchNoteList() {
	const [loading, setLoading] = useState(false);

	const fetchNoteList = useMemoizedFn(async (params: DescribeNoteListRequestParams = {}) => {
		setLoading(true);
		const res = await noteApis.DescribeNoteList({
			page: params.page || 1,
			pageSize: params.pageSize || 10,
		});
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchNoteList,
	};
}

export function useFetchNote() {
	const [loading, setLoading] = useState(false);

	const fetchNote = useMemoizedFn(async (params: DescribeNoteDetailRequestParams = {}) => {
		setLoading(true);
		const res = await noteApis.DescribeNoteDetail(params);
		setLoading(false);
		return res;
	});

	return {
		loading,
		fetchNote,
	};
}
