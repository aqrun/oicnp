import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services/types';
import { TagModel } from '../tag/types';
import { CategoryModel } from '../category/types';
import { Dayjs } from 'dayjs';

export interface BaseNodeModel {
  nid?: number;
  uuid?: string;
  vid?: string;
  bundle?: string;
  title?: string;
  body?: string;
  summary?: string;
  summaryFormat?: string;
  bodyFormat?: string;
  viewed?: number;
  deleted?: string;
  createdBy?: number;
  updatedBy?: number;
  categoryIds?: number[];
  tagIds?: number[];
  tagVids?: string[];
}

export interface NodeModel extends BaseNodeModel {
  publishedAt?: string;
  createdAt?: string;
  updatedAt?: string;
  categories?: Array<CategoryModel>;
  tags?: Array<TagModel>;
}

export interface NodeFieldType extends BaseNodeModel {
  createdAt?: Dayjs;
  publishedAt?: Dayjs;
}


export interface NodeFilters {
  nid?: number;
  nids?: string;
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

export interface DescribeCreateNodeResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateNodeRequestParams = DescribeCreateNodeRequestParams;
export type DescribeUpdateNodeResponseData = DescribeCreateNodeResponseData;
export type DescribeDeleteNodeRequestParams = DescribeCreateNodeRequestParams;
export type DescribeDeleteNodeResponseData = DescribeCreateNodeResponseData;
export type DescribeNodeTagsRequestParams = NodeFilters;

export interface DescribeNodeTagsResponseData {
  tags: Array<TagModel>;
}

export type DescribeNodeCategoriesRequestParams = NodeFilters;

export interface DescribeNodeCategoriesResponseData {
  categories: Array<CategoryModel>;
}

export interface DescribeNodeBodyRequestParams extends NodeFilters {
  _name?: string;
}

export interface NodeBody {
    nid: number;
    summary: string;
    summaryFormat: string;
    body: string;
    bodyFormat: string;
}

export interface DescribeNodeBodyResponseData {
  body: NodeBody;
}
