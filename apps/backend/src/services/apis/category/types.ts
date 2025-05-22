import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface CategoryModel {
  catId?: number;
  catPid?: number;
  catVid?: string;
  catName?: string;
  weight?: number;
  catDesc?: string;
  catDescFormat?: string;
  createdAt?: string;
  updatedAt?: string;
  children?: Array<CategoryModel>;
}

export interface CategoryFilters {
  catId?: number;
  catVid?: string;
  catName?: string;
  weight?: number;
  catDesc?: string;
  catDescFormat?: string;
}

export interface DescribeCategoryDetailRequestParams extends CategoryFilters {
  _name?: string;
}
export interface DescribeCategoryDetailResponseData {
  category: CategoryModel;
  _name?: string;
}

export interface DescribeCategoryListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeCategoryListResponseData extends BaseListResponseData {
  categories: Array<CategoryModel>;
  _name?: string;
}

export interface DescribeCreateCategoryRequestParams extends CategoryModel {
  _name?: string;
}

export interface DescribeCreateCategoryResponseData {
  _name?: string;
}

export type DescribeUpdateCategoryRequestParams = DescribeCreateCategoryRequestParams;
export type DescribeUpdateCategoryResponseData = DescribeCreateCategoryResponseData;
export type DescribeDeleteCategoryRequestParams = DescribeCreateCategoryRequestParams;
export type DescribeDeleteCategoryResponseData = DescribeCreateCategoryResponseData;
