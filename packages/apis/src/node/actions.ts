import {
  createActionService,
} from '@repo/services/action_request';
import {
  DescribeNodeListRequestParams,
  DescribeNodeListResponseData,
  DescribeNodeDetailRequestParams,
  DescribeNodeDetailResponseData,
} from './types';

export const DescribeNodeList = createActionService<
DescribeNodeListRequestParams,
DescribeNodeListResponseData
>('/node/list', 'post');

export const DescribeNodeDetail = createActionService<
DescribeNodeDetailRequestParams,
DescribeNodeDetailResponseData
>('/node/one', 'post');