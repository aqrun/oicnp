import { createService } from './request';
import {
  FetchMenusRequestParams,
  FetchMenusResponseData,
  FetchAuthLoginRequestParams,
  FetchAuthLoginResponseData,
} from './types';

export const fetchMenus = createService<
FetchMenusRequestParams,
FetchMenusResponseData
>('/menus', 'get');

export const fetchLogin = createService<
FetchAuthLoginRequestParams,
FetchAuthLoginResponseData
>('/auth/login', 'post');