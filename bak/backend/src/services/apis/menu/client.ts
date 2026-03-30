'use client';

import { createService } from '../../request';
import {
  DescribeMenuListRequestParams,
  DescribeMenuListResponseData,
  DescribeMenuDetailRequestParams,
  DescribeMenuDetailResponseData,
  DescribeDeleteMenuRequestParams,
  DescribeDeleteMenuResponseData,
  DescribeCreateMenuRequestParams,
  DescribeCreateMenuResponseData,
  DescribeUpdateMenuRequestParams,
  DescribeUpdateMenuResponseData,
  DescribeMenuPermissionsResponseData,
  DescribeMenuPermissionsRequestParams,
  DescribeMenuTreeRequestParams,
  DescribeMenuTreeResponseData,
} from './types';

export const DescribeMenuList = createService<
DescribeMenuListRequestParams,
DescribeMenuListResponseData
>('/menu/list', 'post', { ignoreError: true, });

export const DescribeMenuDetail = createService<
DescribeMenuDetailRequestParams,
DescribeMenuDetailResponseData
>('/menu/one', 'post', { ignoreError: true, });

export const DescribeDeleteMenu = createService<
DescribeDeleteMenuRequestParams,
DescribeDeleteMenuResponseData
>('/menu/remove', 'post');

export const DescribeCreateMenu = createService<
DescribeCreateMenuRequestParams,
DescribeCreateMenuResponseData
>('/menu/add', 'post');

export const DescribeUpdateMenu = createService<
DescribeUpdateMenuRequestParams,
DescribeUpdateMenuResponseData
>('/menu/update', 'post');

export const DescribeMenuPermissions = createService<
DescribeMenuPermissionsRequestParams,
DescribeMenuPermissionsResponseData
>('/menu/permissions', 'post');

/**
 * 获取菜单树
 */
export const DescribeMenuTree = createService<
DescribeMenuTreeRequestParams,
DescribeMenuTreeResponseData
>('/menu/tree', 'post');
