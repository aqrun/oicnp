'use client';

import { createService } from '../../request';
import {
  DescribeTagListRequestParams,
  DescribeTagListResponseData,
  DescribeTagDetailRequestParams,
  DescribeTagDetailResponseData,
  DescribeDeleteTagRequestParams,
  DescribeDeleteTagResponseData,
  DescribeCreateTagRequestParams,
  DescribeCreateTagResponseData,
  DescribeUpdateTagRequestParams,
  DescribeUpdateTagResponseData,
} from './types';

export const DescribeTagList = createService<
DescribeTagListRequestParams,
DescribeTagListResponseData
>('/tag/list', 'post', { ignoreError: true, });

export const DescribeTagDetail = createService<
DescribeTagDetailRequestParams,
DescribeTagDetailResponseData
>('/tag/one', 'post', { ignoreError: true, });

export const DescribeDeleteTag = createService<
DescribeDeleteTagRequestParams,
DescribeDeleteTagResponseData
>('/tag/remove', 'post');

export const DescribeCreateTag = createService<
DescribeCreateTagRequestParams,
DescribeCreateTagResponseData
>('/tag/add', 'post');

export const DescribeUpdateTag = createService<
DescribeUpdateTagRequestParams,
DescribeUpdateTagResponseData
>('/tag/update', 'post');