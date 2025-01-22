import { api } from '~/utils';
import type {
  DescribeUserLoginRequestParams,
  DescribeUserLoginResponseData,
} from './types';

export const DescribeUserLogin = api<
DescribeUserLoginResponseData,
DescribeUserLoginRequestParams
>('describeUserLogin');