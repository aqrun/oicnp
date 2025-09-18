import {
  BaseFilterParams,
  BaseListResponseData,
  BaseResponse,
} from '../../types';

export interface NoteModel {
  id?: number;
  title?: string;
  content?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface NoteFilters {
  id?: number;
  title?: string;
}

export interface DescribeNoteDetailRequestParams extends NoteFilters {
  _name?: string;
}
export interface DescribeNoteDetailResponseData {
  note: NoteModel;
  _name?: string;
}

export interface DescribeNoteListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeNoteListResponseData extends BaseListResponseData {
  notes: Array<NoteModel>;
  _name?: string;
}

export interface DescribeCreateNoteRequestParams extends NoteModel {
  _name?: string;
}

export interface DescribeCreateNoteResponseData extends BaseResponse {
  _name?: string;
}

export type DescribeUpdateNoteRequestParams = DescribeCreateNoteRequestParams;
export type DescribeUpdateNoteResponseData = DescribeCreateNoteResponseData;
export type DescribeDeleteNoteRequestParams = DescribeCreateNoteRequestParams;
export type DescribeDeleteNoteResponseData = DescribeCreateNoteResponseData;
