import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '@repo/services';
import { Dayjs } from 'dayjs';

export interface BaseFileModel {
  fileId?: number;
  uid?: string;
  filename?: string;
  uri?: string;
  storage?: string;
  link?: string;
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
  link?: string;
  mime?: string;
  status?: string;
}

export interface DescribeFileDetailRequestParams extends FileFilters {
  _name?: string;
}
export interface DescribeFileDetailResponseData extends BaseResponse {
  file: UploadFileRes;
  _name?: string;
}

export interface DescribeFileListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeFileListResponseData extends BaseListResponseData {
  files: Array<UploadFileRes>;
  _name?: string;
}

export interface DescribeCreateFileRequestParams extends FileModel {
  _name?: string;
}

export interface DescribeCreateFileResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateFileRequestParams = DescribeCreateFileRequestParams;
export type DescribeUpdateFileResponseData = DescribeCreateFileResponseData;
export type DescribeDeleteFileRequestParams = DescribeCreateFileRequestParams;
export type DescribeDeleteFileResponseData = DescribeCreateFileResponseData;

export interface UploadFileRes {
  id: number;
  name: string;
  size: number;
  fileType: string;
  /// 存储路径
  uri: string;
  /// 图床地址
  link: string;
  /// 内部全路径
  url: string;
  mime: string;
  status: string;
  storage?: string;
  createdAt?: string;
}