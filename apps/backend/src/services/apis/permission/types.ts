import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface PermissionModel {
  permissionId?: number;
  vid?: string;
  pid?: number;
  name?: string;
  weight?: number;
  scope?: string;
  status?: string;
  remark?: string;
  createdAt?: string;
  updatedAt?: string;
  children?: Array<PermissionModel>;
}

export interface DescribePermissionDetailRequestParams extends PermissionModel {
  _name?: string;
}
export interface DescribePermissionDetailResponseData {
  data: PermissionModel;
  _name?: string;
}

export interface DescribePermissionListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribePermissionListResponseData extends BaseListResponseData {
  data: Array<PermissionModel>;
  _name?: string;
}

export interface DescribeCreatePermissionRequestParams extends PermissionModel {
  _name?: string;
}

export interface DescribeCreatePermissionResponseData {
  _name?: string;
}

export type DescribeUpdatePermissionRequestParams = DescribeCreatePermissionRequestParams;
export type DescribeUpdatePermissionResponseData = DescribeCreatePermissionResponseData;
export type DescribeDeletePermissionRequestParams = DescribeCreatePermissionRequestParams;
export type DescribeDeletePermissionResponseData = DescribeCreatePermissionResponseData;
