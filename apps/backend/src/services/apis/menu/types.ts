import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface MenuModel {
  id: string;
  vid?: string;
  pid?: string;
  parentVid?: string;
  path?: string;
  name?: string;
  icon?: string;
  weight?: string;
  api?: string;
  status?: string;
  visible?: string;
  isCache?: string;
  isFrame?: string;
  remark?: string;
  /// 指定权限
  permissionVids?: Array<string>;
  createdAt?: string;
  updateAt?: string;
  deletedAt?: string;
  children?: Array<MenuModel>;
}

export interface DescribeMenuDetailRequestParams {
  _name?: string;
}
export interface DescribeMenuDetailResponseData {
  data: MenuModel;
  _name?: string;
}

export interface DescribeMenuListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeMenuListResponseData extends BaseListResponseData {
  data: Array<MenuModel>;
  _name?: string;
}

export interface DescribeCreateMenuRequestParams extends MenuModel {
  _name?: string;
}

export interface DescribeCreateMenuResponseData {
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
export type DescribeMenuTreeResponseData = MenuTreeItem;
