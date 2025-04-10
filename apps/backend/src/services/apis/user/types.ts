import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface UserModel {
  uid?: string;
  uuid?: string;
  username?: string;
  nickname?: string;
  password?: string;
  salt?: string;
  apiKey?: string;
  resetToken?: string;
  resetSentAt?: string;
  status?: string;
  email?: string;
  gender?: string;
  avatar?: string;
  roleId?: string;
  roles?: Array<string>;
  dptId?: string;
  remark?: string;
  isAdmin?: string;
  phone?: string;
  createdBy?: string;
  updatedBy?: string;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
}

export interface DescribeUserDetailRequestParams {
  uid?: number;
  uuid?: string;
  username?: string;
  nickname?: string;
  password?: string;
  salt?: string;
  apiKey?: string;
  resetToken?: string;
  resetSentAt?: string;
  status?: string;
  email?: string;
  gender?: string;
  avatar?: string;
  roleId?: string;
  roles?: Array<string>;
  dptId?: string;
  remark?: string;
  isAdmin?: string;
  phone?: string;
  createdBy?: string;
  updatedBy?: string;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
  _name?: string;
}
export type DescribeUserDetailResponseData = UserModel;

export interface DescribeUserListRequestParams extends BaseFilterParams {
  uid?: string;
  uuid?: string;
  username?: string;
  nickname?: string;
  password?: string;
  salt?: string;
  apiKey?: string;
  resetToken?: string;
  resetSentAt?: string;
  status?: string;
  email?: string;
  gender?: string;
  avatar?: string;
  roleId?: string;
  roles?: Array<string>;
  dptId?: string;
  remark?: string;
  isAdmin?: string;
  phone?: string;
  createdBy?: string;
  updatedBy?: string;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
  _name?: string;
}

export interface DescribeUserListResponseData extends BaseListResponseData {
  data: Array<UserModel>;
  _name?: string;
}

export interface DescribeCreateUserRequestParams extends UserModel {
  _name?: string;
}

export interface DescribeCreateUserResponseData {
  uid?: string;
  uuid?: string;
  nickname?: string;
  _name?: string;
}

export type DescribeUpdateUserRequestParams = DescribeCreateUserRequestParams;
export type DescribeUpdateUserResponseData = DescribeCreateUserResponseData;
export type DescribeDeleteUserRequestParams = DescribeCreateUserRequestParams;
export type DescribeDeleteUserResponseData = DescribeCreateUserResponseData;
