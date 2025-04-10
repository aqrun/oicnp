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
