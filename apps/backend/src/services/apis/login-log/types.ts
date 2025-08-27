import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface LoginLogModel {
  id?: number;
  loginName?: string;
  net?: string;
  ip?: string;
  location?: string;
  browser?: string;
  os?: string;
  device?: string;
  status?: string;
  message?: string;
  module?: string;
  loginAt?: string;
}

export interface LoginLogFilters {
  id?: number;
  loginName?: string;
  net?: string;
  ip?: string;
  location?: string;
  browser?: string;
  os?: string;
  device?: string;
  status?: string;
  message?: string;
  module?: string;
  loginAt?: string;
}

export interface DescribeLoginLogDetailRequestParams extends LoginLogFilters {
  _name?: string;
}
export interface DescribeLoginLogDetailResponseData {
  log: LoginLogModel;
  _name?: string;
}

export interface DescribeLoginLogListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeLoginLogListResponseData extends BaseListResponseData {
  logs: Array<LoginLogModel>;
  _name?: string;
}

export interface DescribeCreateLoginLogRequestParams extends LoginLogModel {
  _name?: string;
}

export interface DescribeCreateLoginLogResponseData {
  _name?: string;
}

export type DescribeUpdateLoginLogRequestParams = DescribeCreateLoginLogRequestParams;
export type DescribeUpdateLoginLogResponseData = DescribeCreateLoginLogResponseData;
export type DescribeDeleteLoginLogRequestParams = DescribeCreateLoginLogRequestParams;
export type DescribeDeleteLoginLogResponseData = DescribeCreateLoginLogResponseData;
