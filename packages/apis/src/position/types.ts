import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services';

export interface PositionModel {
  positionId?: number;
  vid?: string;
  name?: string;
  weight?: number;
  status?: string;
  remark?: string;
  createdBy?: number;
  updatedBy?: number;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
}

export interface PositionFilters {
  positionId?: number;
  vid?: string;
  name?: string;
  weight?: number;
  status?: string;
  remark?: string;
  createdBy?: number;
  updatedBy?: number;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
}

export interface DescribePositionDetailRequestParams extends PositionFilters {
  _name?: string;
}
export interface DescribePositionDetailResponseData extends BaseResponse {
  position: PositionModel;
  _name?: string;
}

export interface DescribePositionListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribePositionListResponseData extends BaseListResponseData {
  positions: Array<PositionModel>;
  _name?: string;
}

export interface DescribeCreatePositionRequestParams extends PositionModel {
  _name?: string;
}

export interface DescribeCreatePositionResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdatePositionRequestParams = DescribeCreatePositionRequestParams;
export type DescribeUpdatePositionResponseData = DescribeCreatePositionResponseData;
export type DescribeDeletePositionRequestParams = DescribeCreatePositionRequestParams;
export type DescribeDeletePositionResponseData = DescribeCreatePositionResponseData;
