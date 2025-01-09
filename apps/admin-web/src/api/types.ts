import { MenuItem } from '~/types';

export interface FetchMenusRequestParams {
  _name?: string;
}

export interface FetchMenusResponseData {
  menus?: MenuItem[];
}

export interface FetchAuthLoginRequestParams {
  username: string;
  password: string;
  remember: boolean;
}

export interface FetchAuthLoginResponseData {
  username?: string;
  token?: string;
  uuid?: string;
}

export interface FetchUserListRequestParams {
  username: string;
  password: string;
  remember: boolean;
}

export interface UserListData {
  id: string;
  username: string;
  phone: string;
}

export interface FetchUserListResponseData {
  data: UserListData[];
  total: number;
  page: number;
  page_size: number;
}

