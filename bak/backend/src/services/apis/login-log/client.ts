'use client';

import { createService } from '../../request';
import {
  DescribeLoginLogListRequestParams,
  DescribeLoginLogListResponseData,
  DescribeLoginLogDetailRequestParams,
  DescribeLoginLogDetailResponseData,
  DescribeDeleteLoginLogRequestParams,
  DescribeDeleteLoginLogResponseData,
  DescribeCreateLoginLogRequestParams,
  DescribeCreateLoginLogResponseData,
  DescribeUpdateLoginLogRequestParams,
  DescribeUpdateLoginLogResponseData,
} from './types';

export const DescribeLoginLogList = createService<
DescribeLoginLogListRequestParams,
DescribeLoginLogListResponseData
>('/login-log/list', 'post', { ignoreError: true, });

export const DescribeLoginLogDetail = createService<
DescribeLoginLogDetailRequestParams,
DescribeLoginLogDetailResponseData
>('/login-log/one', 'post', { ignoreError: true, });

export const DescribeDeleteLoginLog = createService<
DescribeDeleteLoginLogRequestParams,
DescribeDeleteLoginLogResponseData
>('/login-log/remove', 'post');

export const DescribeCreateLoginLog = createService<
DescribeCreateLoginLogRequestParams,
DescribeCreateLoginLogResponseData
>('/login-log/add', 'post');

export const DescribeUpdateLoginLog = createService<
DescribeUpdateLoginLogRequestParams,
DescribeUpdateLoginLogResponseData
>('/login-log/update', 'post');