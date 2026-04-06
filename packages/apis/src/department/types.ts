import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services';

export interface DepartmentModel {
  id?: number;
  pid?: number;
  vid?: string;
  name?: string;
  weight?: number;
  leader?: string;
  phone?: string;
  email?: string;
  status?: string;
  createdBy?: number;
  updatedBy?: number;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
}

export interface DepartmentFilters {
  id?: number;
  pid?: number;
  vid?: string;
  name?: string;
  weight?: number;
  leader?: string;
  phone?: string;
  email?: string;
  status?: string;
  createdBy?: number;
  updatedBy?: number;
  createdAt?: string;
  updatedAt?: string;
  deletedAt?: string;
}

export interface DescribeDepartmentDetailRequestParams extends DepartmentFilters {
  _name?: string;
}
export interface DescribeDepartmentDetailResponseData extends BaseResponse {
  department: DepartmentModel;
  _name?: string;
}

export interface DescribeDepartmentListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeDepartmentListResponseData extends BaseListResponseData {
  departments: Array<DepartmentModel>;
  _name?: string;
}

export interface DescribeCreateDepartmentRequestParams extends DepartmentModel {
  _name?: string;
}

export interface DescribeCreateDepartmentResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateDepartmentRequestParams = DescribeCreateDepartmentRequestParams;
export type DescribeUpdateDepartmentResponseData = DescribeCreateDepartmentResponseData;
export type DescribeDeleteDepartmentRequestParams = DescribeCreateDepartmentRequestParams;
export type DescribeDeleteDepartmentResponseData = DescribeCreateDepartmentResponseData;
