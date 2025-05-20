import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface NodeModel {
  nid?: number;
  uuid?: string;
  vid?: string;
  bundle?: string;
  title?: string;
  content?: string;
  viewed?: number;
  deleted?: string;
  publishedAt?: string;
  createdBy?: number;
  updatedBy?: number;
  createdAt?: string;
  updatedAt?: string;
}

export interface NodeFilters {
  nid?: number;
  uuid?: string;
  vid?: string;
  bundle?: string;
  title?: string;
  content?: string;
  viewed?: number;
  deleted?: string;
  publishedAt?: string;
  createdBy?: number;
  updatedBy?: number;
}

export interface DescribeNodeDetailRequestParams extends NodeFilters {
  _name?: string;
}
export interface DescribeNodeDetailResponseData {
  node: NodeModel;
  _name?: string;
}

export interface DescribeNodeListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeNodeListResponseData extends BaseListResponseData {
  nodes: Array<NodeModel>;
  _name?: string;
}

export interface DescribeCreateNodeRequestParams extends NodeModel {
  _name?: string;
}

export interface DescribeCreateNodeResponseData {
  _name?: string;
}

export type DescribeUpdateNodeRequestParams = DescribeCreateNodeRequestParams;
export type DescribeUpdateNodeResponseData = DescribeCreateNodeResponseData;
export type DescribeDeleteNodeRequestParams = DescribeCreateNodeRequestParams;
export type DescribeDeleteNodeResponseData = DescribeCreateNodeResponseData;
