import {
  createActionService,
} from '@repo/services/action_request';
import {
  DescribeNodeListRequestParams,
  DescribeNodeListResponseData,
} from './types';

export const DescribeNodeList = createActionService<
DescribeNodeListRequestParams,
DescribeNodeListResponseData
>('/node/list', 'post');