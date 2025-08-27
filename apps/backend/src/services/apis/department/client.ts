'use client';

import { createService } from '../../request';
import {
  DescribeDepartmentListRequestParams,
  DescribeDepartmentListResponseData,
  DescribeDepartmentDetailRequestParams,
  DescribeDepartmentDetailResponseData,
  DescribeDeleteDepartmentRequestParams,
  DescribeDeleteDepartmentResponseData,
  DescribeCreateDepartmentRequestParams,
  DescribeCreateDepartmentResponseData,
  DescribeUpdateDepartmentRequestParams,
  DescribeUpdateDepartmentResponseData,
} from './types';

export const DescribeDepartmentList = createService<
DescribeDepartmentListRequestParams,
DescribeDepartmentListResponseData
>('/department/list', 'post', { ignoreError: true, });

export const DescribeDepartmentDetail = createService<
DescribeDepartmentDetailRequestParams,
DescribeDepartmentDetailResponseData
>('/department/one', 'post', { ignoreError: true, });

export const DescribeDeleteDepartment = createService<
DescribeDeleteDepartmentRequestParams,
DescribeDeleteDepartmentResponseData
>('/department/remove', 'post');

export const DescribeCreateDepartment = createService<
DescribeCreateDepartmentRequestParams,
DescribeCreateDepartmentResponseData
>('/department/add', 'post');

export const DescribeUpdateDepartment = createService<
DescribeUpdateDepartmentRequestParams,
DescribeUpdateDepartmentResponseData
>('/department/update', 'post');