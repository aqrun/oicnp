import { useMemoizedFn } from 'ahooks';
import {
  NodeFilters,
  DescribeNodeDetailRequestParams,
  DescribeNodeBodyRequestParams,
  DescribeNodeTagsRequestParams,
  DescribeNodeCategoriesRequestParams,
  DescribeNodeDetailResponseData,
  DescribeNodeBodyResponseData,
  DescribeNodeTagsResponseData,
  DescribeNodeCategoriesResponseData,
  DescribeNodeListRequestParams,
  DescribeNodeListResponseData,
} from './types';
import {
  DescribeNodeDetail,
  DescribeNodeBody,
  DescribeNodeTags,
  DescribeNodeCategories,
  DescribeNodeList,
} from './client';

export interface NodeAllRes {
  detailRes: DescribeNodeDetailResponseData;
  bodyRes: DescribeNodeBodyResponseData;
  tagRes: DescribeNodeTagsResponseData;
  categoryRes: DescribeNodeCategoriesResponseData;
}

/**
 * 获取节点全部信息
 */
export function useFetchNodeAll() {
  const fetchNodeDetail = useMemoizedFn(async (params: DescribeNodeDetailRequestParams) => {
    const res = await DescribeNodeDetail(params);
    return res;
  });

  const fetchNodeBody = useMemoizedFn(async (params: DescribeNodeBodyRequestParams) => {
    const res = await DescribeNodeBody(params);
    return res;
  });

  const fetchNodeTags = useMemoizedFn(async (params: DescribeNodeTagsRequestParams) => {
    const res = await DescribeNodeTags(params);
    return res;
  });

  const fetchNodeCategories = useMemoizedFn(async (params: DescribeNodeCategoriesRequestParams) => {
    const res = await DescribeNodeCategories(params);
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
      detailRes: allRes?.[0],
      bodyRes: allRes?.[1],
      tagRes: allRes?.[2],
      categoryRes: allRes?.[3],
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

export function useFetchNodeList() {
  const fetchNodeList = useMemoizedFn(async (params: DescribeNodeListRequestParams) => {
    const res = await DescribeNodeList(params);
    return res;
  });

  return {
    fetchNodeList,
  };
}
