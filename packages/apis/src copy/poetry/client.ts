'use client';

import { createService } from '@repo/services/request';
import {
  DescribePoetryListWithChaptersRequestParams,
  DescribePoetryListWithChaptersResponseData,
} from './types';

export const DescribePoetryListWithChapters = createService<
DescribePoetryListWithChaptersRequestParams,
DescribePoetryListWithChaptersResponseData
>('/poetry/list-with-chapters', 'post', { ignoreError: true, });
