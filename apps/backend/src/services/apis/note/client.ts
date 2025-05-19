'use client';

import { createService } from '../../request';
import {
  DescribeNoteListRequestParams,
  DescribeNoteListResponseData,
  DescribeNoteDetailRequestParams,
  DescribeNoteDetailResponseData,
  DescribeDeleteNoteRequestParams,
  DescribeDeleteNoteResponseData,
  DescribeCreateNoteRequestParams,
  DescribeCreateNoteResponseData,
  DescribeUpdateNoteRequestParams,
  DescribeUpdateNoteResponseData,
} from './types';

export const DescribeNoteList = createService<
DescribeNoteListRequestParams,
DescribeNoteListResponseData
>('/note/list', 'post', { ignoreError: true, });

export const DescribeNoteDetail = createService<
DescribeNoteDetailRequestParams,
DescribeNoteDetailResponseData
>('/note/one', 'post', { ignoreError: true, });

export const DescribeDeleteNote = createService<
DescribeDeleteNoteRequestParams,
DescribeDeleteNoteResponseData
>('/note/remove', 'post');

export const DescribeCreateNote = createService<
DescribeCreateNoteRequestParams,
DescribeCreateNoteResponseData
>('/note/add', 'post');

export const DescribeUpdateNote = createService<
DescribeUpdateNoteRequestParams,
DescribeUpdateNoteResponseData
>('/note/update', 'post');