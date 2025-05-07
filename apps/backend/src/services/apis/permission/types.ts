import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface PermissionModel {
  permissionId?: number;
  vid?: string;
  pid?: number;
  name?: string;
  api?: string;
  weight?: number;
  scope?: string;
  status?: string;
  remark?: string;
  createdAt?: string;
  updatedAt?: string;
  children?: Array<PermissionModel>;
}

export interface PermissionTreeItem {
  id: number;
  parent_id: number;
  vid: string;
  api: string;
  weight: number;
  label: string;
  status: string;
  remark: string;
  children?: Array<PermissionTreeItem>;
}

export interface DescribePermissionDetailRequestParams extends PermissionModel {
  _name?: string;
}
export interface DescribePermissionDetailResponseData {
  permission: PermissionModel;
  _name?: string;
}

export interface DescribePermissionListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribePermissionTreeRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribePermissionTreeResponseData {
  permissions: Array<PermissionTreeItem>;
  _name?: string;
}

export interface DescribePermissionListResponseData extends BaseListResponseData {
  permissions: Array<PermissionModel>;
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
