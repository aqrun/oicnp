'use client';

import { createService } from '../../request';
import {
  DescribeFileListRequestParams,
  DescribeFileListResponseData,
  DescribeFileDetailRequestParams,
  DescribeFileDetailResponseData,
  DescribeDeleteFileRequestParams,
  DescribeDeleteFileResponseData,
  DescribeCreateFileRequestParams,
  DescribeCreateFileResponseData,
  DescribeUpdateFileRequestParams,
  DescribeUpdateFileResponseData,
} from './types';

export const DescribeFileList = createService<
DescribeFileListRequestParams,
DescribeFileListResponseData
>('/file/list', 'post', { ignoreError: true, });

export const DescribeFileDetail = createService<
DescribeFileDetailRequestParams,
DescribeFileDetailResponseData
>('/file/one', 'post', { ignoreError: true, });

export const DescribeDeleteFile = createService<
DescribeDeleteFileRequestParams,
DescribeDeleteFileResponseData
>('/file/remove', 'post');

export const DescribeCreateFile = createService<
DescribeCreateFileRequestParams,
DescribeCreateFileResponseData
>('/file/add', 'post');

export const DescribeUpdateFile = createService<
DescribeUpdateFileRequestParams,
DescribeUpdateFileResponseData
>('/file/update', 'post');
