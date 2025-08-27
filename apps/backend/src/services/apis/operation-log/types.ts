import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface OperationLogModel {
  id?: number;
  timeId?: number;
  title?: string;
  businessType?: string;
  method?: string;
  requestMethod?: string;
  operatorType?: string;
  name?: string;
  departmentName?: string;
  url?: string;
  ip?: string;
  location?: string;
  param?: string;
  pathParam?: string;
  jsonResult?: string;
  status?: string;
  errorMessage?: string;
  duration?: number;
  createdAt?: string;
}

export interface OperationLogFilters {
  id?: number;
  timeId?: number;
  title?: string;
  businessType?: string;
  method?: string;
  requestMethod?: string;
  operatorType?: string;
  name?: string;
  departmentName?: string;
  url?: string;
  ip?: string;
  location?: string;
  param?: string;
  pathParam?: string;
  jsonResult?: string;
  status?: string;
  errorMessage?: string;
  duration?: number;
  createdAt?: string;
}

export interface DescribeOperationLogDetailRequestParams extends OperationLogFilters {
  _name?: string;
}
export interface DescribeOperationLogDetailResponseData {
  log: OperationLogModel;
  _name?: string;
}

export interface DescribeOperationLogListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeOperationLogListResponseData extends BaseListResponseData {
  logs: Array<OperationLogModel>;
  _name?: string;
}

export interface DescribeCreateOperationLogRequestParams extends OperationLogModel {
  _name?: string;
}

export interface DescribeCreateOperationLogResponseData {
  _name?: string;
}

export type DescribeUpdateOperationLogRequestParams = DescribeCreateOperationLogRequestParams;
export type DescribeUpdateOperationLogResponseData = DescribeCreateOperationLogResponseData;
export type DescribeDeleteOperationLogRequestParams = DescribeCreateOperationLogRequestParams;
export type DescribeDeleteOperationLogResponseData = DescribeCreateOperationLogResponseData;
