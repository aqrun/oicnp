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
  DescribeCreateUserRequestParams,
  DescribeCreateUserResponseData,
  DescribeUserDetailRequestParams,
  DescribeUserDetailResponseData,
  DescribeUpdateUserRequestParams,
  DescribeUpdateUserResponseData,
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
>('/user/list', 'post', { ignoreError: true, });

export const DescribeUserDetail = createService<
DescribeUserDetailRequestParams,
DescribeUserDetailResponseData
>('/user/one', 'post', { ignoreError: true, });

export const DescribeDeleteUser = createService<
DescribeDeleteUserRequestParams,
DescribeDeleteUserResponseData
>('/user/remove', 'post');

export const DescribeCreateUser = createService<
DescribeCreateUserRequestParams,
DescribeCreateUserResponseData
>('/user/add', 'post');

export const DescribeUpdateUser = createService<
DescribeUpdateUserRequestParams,
DescribeUpdateUserResponseData
>('/user/update', 'post');