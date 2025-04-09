import { MenuItem } from '@/types';

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
  vid?: string;
  _name?: string;
}

export interface DescribeMenuListResponseData {
  menus?: MenuItem[];
}

export interface MenuTreeItem {
  id: number;
  mid: string;
  pid: string;
  path: string;
  label: string;
  weight: number;
  icon: string;
  isActive?: boolean;
  children: MenuTreeItem[];
}

export type DescribeMenuTreeResponseData = MenuTreeItem;

export interface DescribeUserRequestParams {
  uuid: string;
}

export interface DescribeUserResponseData {
  uid: number;
  uuid: string;
  username: string;
  nickname: string;
  api_key: string;
  status: string;
  is_admin: string;
  remark: string;
  avatar: string;
  gender: string;
  email: string;
  phone: string;
  last_login_ip: string,
  last_login_at: string,
}

export interface DescribeLoginRequestParams {
  username?: string;
  email: string;
  password: string;
  remember: boolean;
}

export interface DescribeLoginResponseData {
  code?: string;
  message?: string;
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
  uid: string;
  uuid: string;
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

export interface DescribeCreateUserRequestParams {
  username?: string;
  email?: string;
  nickname?: string;
  password?: string;
  status?: string;
  isAdmin?: string;
}
export interface DescribeCreateUserResponseData {
  uid?: number;
  uuid?: string;
  username?: string;
  nickname?: string;
}

export interface DescribeUserDetailRequestParams extends BaseFilterParams{
  uid?: number;
  uuid?: string;
  username?: string;
  _name?: string;
}

export interface DescribeUserDetailResponseData {
  uid?: number;
  uuid?: string;
  username?: string;
  nickname?: string;
  email?: string;
  status?: string;
  isAdmin?: string;
}

export interface DescribeUpdateUserRequestParams {
  uid?: number;
  username?: string;
  email?: string;
  nickname?: string;
  password?: string;
  status?: string;
  isAdmin?: string;
}

export interface DescribeUpdateUserResponseData {
  uid?: number;
  uuid?: string;
  username?: string;
  nickname?: string;
  email?: string;
  status?: string;
  isAdmin?: string;
}

