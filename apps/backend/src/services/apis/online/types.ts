import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

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
  id?: number;
  title?: string;
}

export interface DescribeOnlineListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeOnlineListResponseData extends BaseListResponseData {
  onlines: Array<OnlineModel>;
  _name?: string;
}
