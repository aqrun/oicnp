import {
  createActionService,
} from '@repo/services/action_request';
import {
  DescribePoetryListPageDataResponseData,
  DescribePoetryListPageDataRequestParams,
} from './types';

export const DescribePoetryListPageData = createActionService<
DescribePoetryListPageDataRequestParams,
DescribePoetryListPageDataResponseData
>('/poetry/list-page-data', 'post');