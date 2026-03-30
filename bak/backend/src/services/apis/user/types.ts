import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '../../types';
import { RoleModel } from '../role/types';

export interface UserModel {
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
export interface DescribeUserDetailResponseData extends BaseResponse {
  user: UserModel;
}

export interface DescribeUserListRequestParams extends BaseFilterParams {
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

export interface DescribeUserListResponseData extends BaseListResponseData {
  users: Array<UserModel>;
  _name?: string;
}

export interface DescribeCreateUserRequestParams extends UserModel {
  _name?: string;
}

export interface DescribeCreateUserResponseData extends BaseResponse {
  uid?: number;
  uuid?: string;
  nickname?: string;
  _name?: string;
}

export type DescribeUpdateUserRequestParams = DescribeCreateUserRequestParams;
export type DescribeUpdateUserResponseData = DescribeCreateUserResponseData;
export type DescribeDeleteUserRequestParams = DescribeCreateUserRequestParams;
export type DescribeDeleteUserResponseData = DescribeCreateUserResponseData;

export interface DescribeUserRolesRequestParams {
  uid?: number | string;
}

export interface DescribeUserRolesResponseData {
  roles: Array<RoleModel>;
}

export interface DescribeLoginRequestParams {
  username?: string;
  email: string;
  password: string;
  remember: boolean;
  captchaId?: string;
  captcha?: string;
}

export interface DescribeLoginResponseData extends BaseResponse {
  username?: string;
  token?: string;
  uuid?: string;
  uid?: number;
}

export interface DescribeAuthInfoRequestParams {
  _name?: string;
}

export interface DescribeAuthInfoResponseData extends BaseResponse {
  user: DescribeLoginResponseData;
}
