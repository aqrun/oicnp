import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services/types';

export interface TagModel {
  tagId?: number;
  tagVid?: string;
  tagName?: string;
  weight?: number;
  tagCount?: number;
}

export interface TagFilters {
  tagId?: number;
  tagVid?: string;
  tagName?: string;
  weight?: number;
  tagCount?: number;
}

export interface DescribeTagDetailRequestParams extends TagFilters {
  _name?: string;
}
export interface DescribeTagDetailResponseData {
  tag: TagModel;
  _name?: string;
}

export interface DescribeTagListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeTagListResponseData extends BaseListResponseData {
  tags: Array<TagModel>;
  _name?: string;
}

export interface DescribeCreateTagRequestParams extends TagModel {
  _name?: string;
}

export interface DescribeCreateTagResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateTagRequestParams = DescribeCreateTagRequestParams;
export type DescribeUpdateTagResponseData = DescribeCreateTagResponseData;
export type DescribeDeleteTagRequestParams = DescribeCreateTagRequestParams;
export type DescribeDeleteTagResponseData = DescribeCreateTagResponseData;
