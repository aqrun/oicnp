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
