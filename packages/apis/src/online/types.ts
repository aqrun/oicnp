import {
  BaseResponse,
  BaseFilterParams,
  BaseListResponseData,
} from '@repo/services';

export interface OnlineModel {
  uid: number,
  tokenId: string,
  tokenExpire: number,
  loginAt: string,
  username: string,
  dptName: string,
  net: string,
  ip: string,
  location: string,
  device: string,
  browser: string,
  os: string,
}

export interface OnlineFilters {
  uid?: number;
  tokenId?: string;
}

export interface DescribeOnlineListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeOnlineListResponseData extends BaseListResponseData {
  onlineList: Array<OnlineModel>;
}

export interface DescribeForceLogoutRequestParams extends OnlineFilters {
  _name?: string;
}

export interface DescribeForceLogoutResponseData extends BaseResponse {
  res: string;
}