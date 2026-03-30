'use client';

import { createService } from '../../request';
import {
  DescribeCategoryListRequestParams,
  DescribeCategoryListResponseData,
  DescribeCategoryDetailRequestParams,
  DescribeCategoryDetailResponseData,
  DescribeDeleteCategoryRequestParams,
  DescribeDeleteCategoryResponseData,
  DescribeCreateCategoryRequestParams,
  DescribeCreateCategoryResponseData,
  DescribeUpdateCategoryRequestParams,
  DescribeUpdateCategoryResponseData,
} from './types';

export const DescribeCategoryList = createService<
DescribeCategoryListRequestParams,
DescribeCategoryListResponseData
>('/category/list', 'post', { ignoreError: true, });

export const DescribeCategoryDetail = createService<
DescribeCategoryDetailRequestParams,
DescribeCategoryDetailResponseData
>('/category/one', 'post', { ignoreError: true, });

export const DescribeDeleteCategory = createService<
DescribeDeleteCategoryRequestParams,
DescribeDeleteCategoryResponseData
>('/category/remove', 'post');

export const DescribeCreateCategory = createService<
DescribeCreateCategoryRequestParams,
DescribeCreateCategoryResponseData
>('/category/add', 'post');

export const DescribeUpdateCategory = createService<
DescribeUpdateCategoryRequestParams,
DescribeUpdateCategoryResponseData
>('/category/update', 'post');