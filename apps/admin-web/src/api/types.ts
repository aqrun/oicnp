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

