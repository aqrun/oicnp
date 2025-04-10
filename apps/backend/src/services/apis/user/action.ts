import { createFetcher } from '../../fetcher';
import {
  DescribeUserDetailRequestParams,
  DescribeUserDetailResponseData,
} from './types';

export const DescribeUserDetail = createFetcher<
DescribeUserDetailRequestParams,
DescribeUserDetailResponseData
>('/user/one', 'post');
