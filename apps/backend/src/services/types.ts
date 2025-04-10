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
