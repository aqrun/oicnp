import { createService } from './request';
import {
  FetchMenusRequestParams,
  FetchMenusResponseData,
  FetchAuthLoginRequestParams,
  FetchAuthLoginResponseData,
  FetchUserListRequestParams,
  FetchUserListResponseData,
} from './types';

export const fetchMenus = createService<
FetchMenusRequestParams,
FetchMenusResponseData
>('/menus', 'get');

export const fetchLogin = createService<
FetchAuthLoginRequestParams,
FetchAuthLoginResponseData
>('/auth/login', 'post');

export const fetchUserList = createService<
FetchUserListRequestParams,
FetchUserListResponseData
>('/user/list', 'post');