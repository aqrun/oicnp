'use client';

import { createService } from '../../request';
import {
  DescribePositionListRequestParams,
  DescribePositionListResponseData,
  DescribePositionDetailRequestParams,
  DescribePositionDetailResponseData,
  DescribeDeletePositionRequestParams,
  DescribeDeletePositionResponseData,
  DescribeCreatePositionRequestParams,
  DescribeCreatePositionResponseData,
  DescribeUpdatePositionRequestParams,
  DescribeUpdatePositionResponseData,
} from './types';

export const DescribePositionList = createService<
DescribePositionListRequestParams,
DescribePositionListResponseData
>('/position/list', 'post', { ignoreError: true, });

export const DescribePositionDetail = createService<
DescribePositionDetailRequestParams,
DescribePositionDetailResponseData
>('/position/one', 'post', { ignoreError: true, });

export const DescribeDeletePosition = createService<
DescribeDeletePositionRequestParams,
DescribeDeletePositionResponseData
>('/position/remove', 'post');

export const DescribeCreatePosition = createService<
DescribeCreatePositionRequestParams,
DescribeCreatePositionResponseData
>('/position/add', 'post');

export const DescribeUpdatePosition = createService<
DescribeUpdatePositionRequestParams,
DescribeUpdatePositionResponseData
>('/position/update', 'post');