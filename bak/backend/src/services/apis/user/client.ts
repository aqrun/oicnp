'use client';

import { createService } from '../../request';
import {
  DescribeUserListRequestParams,
  DescribeUserListResponseData,
  DescribeUserDetailRequestParams,
  DescribeUserDetailResponseData,
  DescribeDeleteUserRequestParams,
  DescribeDeleteUserResponseData,
  DescribeCreateUserRequestParams,
  DescribeCreateUserResponseData,
  DescribeUpdateUserRequestParams,
  DescribeUpdateUserResponseData,
  DescribeUserRolesRequestParams,
  DescribeUserRolesResponseData,
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
  DescribeAuthInfoRequestParams,
  DescribeAuthInfoResponseData,
} from './types';


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

export const DescribeUserRoles = createService<
DescribeUserRolesRequestParams,
DescribeUserRolesResponseData
>('/user/roles', 'post');

export const DescribeAuthLogin = createService<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');

/**
 * 获取当前登录用户信息
 */
export const DescribeAuthInfo = createService<
DescribeAuthInfoRequestParams,
DescribeAuthInfoResponseData
>('/auth/info', 'post');
