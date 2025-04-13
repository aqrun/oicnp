import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface RoleModel {
  roleId?: string;
  vid?: string;
  name?: string;
  weight?: number;
  scope?: string;
  status?: string;
  remark?: string;
  createdAt?: string;
  updatedAt?: string;
  permissionVids?: Array<string>;
  permissionIds?: Array<string>;
}

export interface DescribeRoleDetailRequestParams {
  _name?: string;
}
export interface DescribeRoleDetailResponseData {
  data: RoleModel;
  _name?: string;
}

export interface DescribeRoleListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeRoleListResponseData extends BaseListResponseData {
  data: Array<RoleModel>;
  _name?: string;
}

export interface DescribeCreateRoleRequestParams extends RoleModel {
  _name?: string;
}

export interface DescribeCreateRoleResponseData {
  _name?: string;
}

export type DescribeUpdateRoleRequestParams = DescribeCreateRoleRequestParams;
export type DescribeUpdateRoleResponseData = DescribeCreateRoleResponseData;
export type DescribeDeleteRoleRequestParams = DescribeCreateRoleRequestParams;
export type DescribeDeleteRoleResponseData = DescribeCreateRoleResponseData;
