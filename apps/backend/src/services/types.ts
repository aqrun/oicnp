export interface BaseResponse {
  code: string;
  data: unknown;
  message: string;
}

export interface BaseFilterParams {
  page?: number;
  pageSize?: number;
  orderBy?: string;
  order?: string;
}

export interface BaseListResponseData {
  total: number;
  page: number;
  pageSize: number;
}

export interface DescribeLoginRequestParams {
  username?: string;
  email: string;
  password: string;
  remember: boolean;
  captchaId?: string;
  captcha?: string;
}

export interface DescribeLoginResponseData {
  code?: string;
  message?: string;
  username?: string;
  token?: string;
  uuid?: string;
}
