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

export interface BaseListResponseData extends BaseResponse {
  total: number;
  page: number;
  pageSize: number;
}

export interface FailModel {
  code?: string;
  message?: string;
  action?: string;
  requestId?: string;
}
