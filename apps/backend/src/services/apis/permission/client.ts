'use client';

import { createService } from '../../request';
import {
  DescribePermissionListRequestParams,
  DescribePermissionListResponseData,
  DescribePermissionDetailRequestParams,
  DescribePermissionDetailResponseData,
  DescribeDeletePermissionRequestParams,
  DescribeDeletePermissionResponseData,
  DescribeCreatePermissionRequestParams,
  DescribeCreatePermissionResponseData,
  DescribeUpdatePermissionRequestParams,
  DescribeUpdatePermissionResponseData,
  DescribePermissionTreeRequestParams,
  DescribePermissionTreeResponseData,
} from './types';

export const DescribePermissionList = createService<
DescribePermissionListRequestParams,
DescribePermissionListResponseData
>('/permission/list', 'post', { ignoreError: true, });

export const DescribePermissionTree = createService<
DescribePermissionTreeRequestParams,
DescribePermissionTreeResponseData
>('/permission/tree', 'post', { ignoreError: true, });

export const DescribePermissionDetail = createService<
DescribePermissionDetailRequestParams,
DescribePermissionDetailResponseData
>('/permission/one', 'post', { ignoreError: true, });

export const DescribeDeletePermission = createService<
DescribeDeletePermissionRequestParams,
DescribeDeletePermissionResponseData
>('/permission/remove', 'post');

export const DescribeCreatePermission = createService<
DescribeCreatePermissionRequestParams,
DescribeCreatePermissionResponseData
>('/permission/add', 'post');

export const DescribeUpdatePermission = createService<
DescribeUpdatePermissionRequestParams,
DescribeUpdatePermissionResponseData
>('/permission/update', 'post');
