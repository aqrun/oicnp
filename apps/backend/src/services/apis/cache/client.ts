'use client';

import { createService } from '../../request';
import {
  DescribeCacheListRequestParams,
  DescribeCacheListResponseData,
  DescribeCacheDetailRequestParams,
  DescribeCacheDetailResponseData,
  DescribeCacheScopeListRequestParams,
  DescribeCacheScopeListResponseData,
} from './types';

export const DescribeCacheList = createService<
DescribeCacheListRequestParams,
DescribeCacheListResponseData
>('/cache/list', 'post', { ignoreError: true, });

export const DescribeCacheDetail = createService<
DescribeCacheDetailRequestParams,
DescribeCacheDetailResponseData
>('/cache/one', 'post', { ignoreError: true, });

export const DescribeCacheScopeList = createService<
DescribeCacheScopeListRequestParams,
DescribeCacheScopeListResponseData
>('/cache/scope-list', 'post', { ignoreError: true, });