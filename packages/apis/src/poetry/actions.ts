import {
  createActionService,
} from '@repo/services/action_request';
import {
  DescribePoetryListPageDataResponseData,
  DescribePoetryListPageDataRequestParams,
  DescribePoetryListWithChaptersRequestParams,
  DescribePoetryListWithChaptersResponseData,
} from './types';

export const DescribePoetryListPageData = createActionService<
DescribePoetryListPageDataRequestParams,
DescribePoetryListPageDataResponseData
>('/poetry/list-page-data', 'post');

export const DescribePoetryListWithChapters = createActionService<
DescribePoetryListWithChaptersRequestParams,
DescribePoetryListWithChaptersResponseData
>('/poetry/list-with-chapters', 'post');