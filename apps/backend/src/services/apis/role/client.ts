'use client';

import { createService } from '../../request';
import {
  DescribeRoleListRequestParams,
  DescribeRoleListResponseData,
  DescribeRoleDetailRequestParams,
  DescribeRoleDetailResponseData,
  DescribeDeleteRoleRequestParams,
  DescribeDeleteRoleResponseData,
  DescribeCreateRoleRequestParams,
  DescribeCreateRoleResponseData,
  DescribeUpdateRoleRequestParams,
  DescribeUpdateRoleResponseData,
} from './types';

export const DescribeRoleList = createService<
DescribeRoleListRequestParams,
DescribeRoleListResponseData
>('/role/list', 'post', { ignoreError: true, });

export const DescribeRoleDetail = createService<
DescribeRoleDetailRequestParams,
DescribeRoleDetailResponseData
>('/role/one', 'post', { ignoreError: true, });

export const DescribeDeleteRole = createService<
DescribeDeleteRoleRequestParams,
DescribeDeleteRoleResponseData
>('/role/remove', 'post');

export const DescribeCreateRole = createService<
DescribeCreateRoleRequestParams,
DescribeCreateRoleResponseData
>('/role/add', 'post');

export const DescribeUpdateRole = createService<
DescribeUpdateRoleRequestParams,
DescribeUpdateRoleResponseData
>('/role/update', 'post');