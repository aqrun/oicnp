import { useMemoizedFn } from 'ahooks';
import { nodeApis } from '#src/api';
import type {
	NodeFilters,
	DescribeNodeDetailRequestParams,
	DescribeNodeBodyRequestParams,
	DescribeNodeTagsRequestParams,
	DescribeNodeCategoriesRequestParams,
	DescribeNodeDetailResponseData,
	DescribeNodeBodyResponseData,
	DescribeNodeTagsResponseData,
	DescribeNodeCategoriesResponseData,
} from '@repo/apis';

export interface NodeAllRes {
	detailRes: DescribeNodeDetailResponseData;
	bodyRes: DescribeNodeBodyResponseData;
	tagRes: DescribeNodeTagsResponseData;
	categoryRes: DescribeNodeCategoriesResponseData;
}

/**
 * 获取节点全部信息（详情、正文、标签、分类）
 */
export function useFetchNodeAll() {
	const fetchNodeDetail = useMemoizedFn(async (params: DescribeNodeDetailRequestParams) => {
		const res = await nodeApis.DescribeNodeDetail(params);
		return res;
	});

	const fetchNodeBody = useMemoizedFn(async (params: DescribeNodeBodyRequestParams) => {
		const res = await nodeApis.DescribeNodeBody(params);
		return res;
	});

	const fetchNodeTags = useMemoizedFn(async (params: DescribeNodeTagsRequestParams) => {
		const res = await nodeApis.DescribeNodeTags(params);
		return res;
	});

	const fetchNodeCategories = useMemoizedFn(async (params: DescribeNodeCategoriesRequestParams) => {
		const res = await nodeApis.DescribeNodeCategories(params);
		return res;
	});

	const fetchNodeAll = useMemoizedFn(async (params: NodeFilters) => {
		const requests = [
			fetchNodeDetail({ nid: params?.nid }),
			fetchNodeBody({ nid: params?.nid }),
			fetchNodeTags({ nid: params?.nid }),
			fetchNodeCategories({ nid: params?.nid }),
		] as const;
		const allRes = await Promise.all(requests);
		const resData: NodeAllRes = {
			detailRes: allRes[0],
			bodyRes: allRes[1],
			tagRes: allRes[2],
			categoryRes: allRes[3],
		};
		return resData;
	});

	return {
		fetchNodeAll,
		fetchNodeDetail,
		fetchNodeBody,
		fetchNodeTags,
		fetchNodeCategories,
	};
}
