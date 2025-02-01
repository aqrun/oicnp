'use client';

import { createService } from './request';
import {
  DescribeMenuListRequestParams,
  DescribeMenuListResponseData,
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
  DescribeUserListRequestParams,
  DescribeUserListResponseData,
  DescribeDeleteUserRequestParams,
  DescribeDeleteUserResponseData,
} from './types';

export const DescribeMenuList = createService<
DescribeMenuListRequestParams,
DescribeMenuListResponseData
>('/menus', 'get');

export const DescribeLogin = createService<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');

export const DescribeUserList = createService<
DescribeUserListRequestParams,
DescribeUserListResponseData
>('/user/list', 'post');

export const DescribeDeleteUser = createService<
DescribeDeleteUserRequestParams,
DescribeDeleteUserResponseData
>('/user/remove', 'post');