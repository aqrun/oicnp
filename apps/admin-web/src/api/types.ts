import { MenuItem } from '~/types';

export interface BaseFilterParams {
  page?: number;
  page_size?: number;
  order_by?: string;
  order?: string;
}

export interface BaseListResponseData {
  total: number;
  page: number;
  page_size: number;
}

export interface DescribeMenuListRequestParams {
  _name?: string;
}

export interface DescribeMenuListResponseData {
  menus?: MenuItem[];
}

export interface DescribeLoginRequestParams {
  username: string;
  password: string;
  remember: boolean;
}

export interface DescribeLoginResponseData {
  username?: string;
  token?: string;
  uuid?: string;
}

export interface DescribeUserListRequestParams extends BaseFilterParams{
  uuid?: string;
  username?: string;
  _name?: string;
}

export interface UserListData {
  id: string;
  username: string;
  phone: string;
}

export interface DescribeUserListResponseData extends BaseListResponseData {
  data: UserListData[];
}

export interface DescribeDeleteUserRequestParams {
  uid?: number | string;
  uuid?: string;
}

export interface DescribeDeleteUserResponseData {
  res?: string;
}

