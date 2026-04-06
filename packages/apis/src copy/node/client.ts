'use client';

import { createService } from '@repo/services/request';
import {
  DescribeNodeListRequestParams,
  DescribeNodeListResponseData,
  DescribeNodeDetailRequestParams,
  DescribeNodeDetailResponseData,
  DescribeDeleteNodeRequestParams,
  DescribeDeleteNodeResponseData,
  DescribeCreateNodeRequestParams,
  DescribeCreateNodeResponseData,
  DescribeUpdateNodeRequestParams,
  DescribeUpdateNodeResponseData,
  DescribeNodeTagsRequestParams,
  DescribeNodeTagsResponseData,
  DescribeNodeCategoriesRequestParams,
  DescribeNodeCategoriesResponseData,
  DescribeNodeBodyRequestParams,
  DescribeNodeBodyResponseData,
} from './types';

export const DescribeNodeList = createService<
DescribeNodeListRequestParams,
DescribeNodeListResponseData
>('/node/list', 'post', { ignoreError: true, });

export const DescribeNodeDetail = createService<
DescribeNodeDetailRequestParams,
DescribeNodeDetailResponseData
>('/node/one', 'post', { ignoreError: true, });

export const DescribeDeleteNode = createService<
DescribeDeleteNodeRequestParams,
DescribeDeleteNodeResponseData
>('/node/remove', 'post');

export const DescribeCreateNode = createService<
DescribeCreateNodeRequestParams,
DescribeCreateNodeResponseData
>('/node/add', 'post');

export const DescribeUpdateNode = createService<
DescribeUpdateNodeRequestParams,
DescribeUpdateNodeResponseData
>('/node/update', 'post');

export const DescribeNodeTags = createService<
DescribeNodeTagsRequestParams,
DescribeNodeTagsResponseData
>('/node/tags', 'post');

export const DescribeNodeCategories = createService<
DescribeNodeCategoriesRequestParams,
DescribeNodeCategoriesResponseData
>('/node/categories', 'post');

export const DescribeNodeBody = createService<
DescribeNodeBodyRequestParams,
DescribeNodeBodyResponseData
>('/node/body', 'post');
