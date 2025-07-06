import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';
import { Dayjs } from 'dayjs';

export interface BaseFileModel {
  fileId?: number;
  uid?: string;
  filename?: string;
  uri?: string;
  storage?: string;
  mime?: string;
  status?: string;
  createdBy?: number;
  updatedBy?: number;
}

export interface FileModel extends BaseFileModel {
  publishedAt?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface FileFieldType extends BaseFileModel {
  createdAt?: Dayjs;
}


export interface FileFilters {
  fileId?: number;
  uid?: string;
  filename?: string;
  uri?: string;
  storage?: string;
  mime?: string;
  status?: string;
}

export interface DescribeFileDetailRequestParams extends FileFilters {
  _name?: string;
}
export interface DescribeFileDetailResponseData {
  file: FileModel;
  _name?: string;
}

export interface DescribeFileListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeFileListResponseData extends BaseListResponseData {
  files: Array<FileModel>;
  _name?: string;
}

export interface DescribeCreateFileRequestParams extends FileModel {
  _name?: string;
}

export interface DescribeCreateFileResponseData {
  _name?: string;
}

export type DescribeUpdateFileRequestParams = DescribeCreateFileRequestParams;
export type DescribeUpdateFileResponseData = DescribeCreateFileResponseData;
export type DescribeDeleteFileRequestParams = DescribeCreateFileRequestParams;
export type DescribeDeleteFileResponseData = DescribeCreateFileResponseData;