import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services/types';

export interface BasePoetryModel {
  id?: number;
  uuid?: string;
  title?: string;
  authorId?: number;
  dynasty?: string;
  weight?: number;
  hotWeight?: number;
  content?: string;
  wordCount?: number;
  tags?: string;
  description?: string;
}

export interface PoetryModel extends BasePoetryModel {
  createdAt?: string;
  updatedAt?: string;
}

export interface PoetryListPageDataModel extends PoetryModel {
  authorUuid?: string;
  authorName?: string;
  isBook?: string;
}

export interface PoetryFilters extends BaseFilterParams {
  poetryAmount?: number;
  chapterAmount?: number;
  id?: number;
  uuid?: string;
  title?: string;
  authorId?: number;
  dynasty?: string;
  weight?: number;
  hotWeight?: number;
  content?: string;
  wordCount?: number;
  tags?: string;
  description?: string;
}

export interface DescribePoetryDetailRequestParams extends PoetryFilters {
  _name?: string;
}
export interface DescribePoetryDetailResponseData extends BaseResponse {
  poetry: PoetryModel;
  _name?: string;
}

export interface DescribePoetryListRequestParams extends PoetryFilters {
  _name?: string;
}

export interface DescribePoetryListResponseData extends BaseListResponseData {
  poetry: Array<PoetryModel>;
  _name?: string;
}

export interface DescribeCreatePoetryRequestParams extends PoetryModel {
  _name?: string;
}

export interface DescribeCreatePoetryResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdatePoetryRequestParams = DescribeCreatePoetryRequestParams;
export type DescribeUpdatePoetryResponseData = DescribeCreatePoetryResponseData;
export type DescribeDeletePoetryRequestParams = DescribeCreatePoetryRequestParams;
export type DescribeDeletePoetryResponseData = DescribeCreatePoetryResponseData;

export interface ChapterModel {
  id?: number;
  uuid?: string;
  poetryId?: number;
  pid?: number;
  title?: string;
  wordCount?: number;
  weight?: number;
  createdAt?: string;
  updatedAt?: string;
  description?: string;
  content?: string;
}

export interface DescribePoetryBodyRequestParams extends PoetryFilters {
  _name?: string;
}

export interface DescribePoetryListPageDataRequestParams extends PoetryFilters {
  _name?: string;
}

export interface DescribePoetryListPageDataResponseData extends BaseResponse {
  entry: {
    poetry_list: Array<PoetryListPageDataModel>;
    chapter_list: Array<ChapterModel>;
  };
}