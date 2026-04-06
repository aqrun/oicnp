import {
  BaseResponse,
  BaseFilterParams,
  BaseListResponseData,
} from '@repo/services';
import {
  PermissionModel,
} from '../permission/types';

export interface MenuModel {
  id: number;
  vid?: string;
  pid?: number;
  parentVid?: string;
  path?: string;
  name?: string;
  icon?: string;
  weight?: number;
  api?: string;
  status?: string;
  visible?: string;
  isCache?: string;
  isFrame?: string;
  remark?: string;
  /**
   * 指定权限
   */
  permissionVids?: Array<string>;
  permissionIds?: Array<number>;
  createdAt?: string;
  updateAt?: string;
  deletedAt?: string;
  children?: Array<MenuModel>;
}

export interface DescribeMenuDetailRequestParams extends Partial<MenuModel> {
  _name?: string;
}
export interface DescribeMenuDetailResponseData extends BaseResponse {
  menu: MenuModel;
  _name?: string;
}

export interface DescribeMenuListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeMenuListResponseData extends BaseListResponseData {
  menus: Array<MenuModel>;
  _name?: string;
}

export interface DescribeCreateMenuRequestParams extends Partial<MenuModel> {
  _name?: string;
}

export interface DescribeCreateMenuResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateMenuRequestParams = DescribeCreateMenuRequestParams;
export type DescribeUpdateMenuResponseData = DescribeCreateMenuResponseData;
export type DescribeDeleteMenuRequestParams = DescribeCreateMenuRequestParams;
export type DescribeDeleteMenuResponseData = DescribeCreateMenuResponseData;

export interface MenuTreeItem {
  id: number;
  mid: string;
  pid: string;
  path: string;
  label: string;
  weight: number;
  icon: string;
  isActive?: boolean;
  children: MenuTreeItem[];
}

export interface DescribeMenuTreeRequestParams {
  vid?: string;
}
export interface DescribeMenuTreeResponseData extends BaseResponse{
  menus: Array<MenuTreeItem>;
}

export interface DescribeMenuPermissionsRequestParams {
  id: number;
}

export interface DescribeMenuPermissionsResponseData extends BaseResponse {
  permissions: Array<PermissionModel>;
}
