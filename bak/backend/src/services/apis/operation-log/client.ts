'use client';

import { createService } from '../../request';
import {
  DescribeOperationLogListRequestParams,
  DescribeOperationLogListResponseData,
  DescribeOperationLogDetailRequestParams,
  DescribeOperationLogDetailResponseData,
  DescribeDeleteOperationLogRequestParams,
  DescribeDeleteOperationLogResponseData,
  DescribeCreateOperationLogRequestParams,
  DescribeCreateOperationLogResponseData,
  DescribeUpdateOperationLogRequestParams,
  DescribeUpdateOperationLogResponseData,
} from './types';

export const DescribeOperationLogList = createService<
DescribeOperationLogListRequestParams,
DescribeOperationLogListResponseData
>('/operation-log/list', 'post', { ignoreError: true, });

export const DescribeOperationLogDetail = createService<
DescribeOperationLogDetailRequestParams,
DescribeOperationLogDetailResponseData
>('/operation-log/one', 'post', { ignoreError: true, });

export const DescribeDeleteOperationLog = createService<
DescribeDeleteOperationLogRequestParams,
DescribeDeleteOperationLogResponseData
>('/operation-log/remove', 'post');

export const DescribeCreateOperationLog = createService<
DescribeCreateOperationLogRequestParams,
DescribeCreateOperationLogResponseData
>('/operation-log/add', 'post');

export const DescribeUpdateOperationLog = createService<
DescribeUpdateOperationLogRequestParams,
DescribeUpdateOperationLogResponseData
>('/operation-log/update', 'post');