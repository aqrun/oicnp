'use client';

import { createService } from '../../request';
import {
  DescribeOnlineListRequestParams,
  DescribeOnlineListResponseData,
} from './types';

export const DescribeOnlineList = createService<
DescribeOnlineListRequestParams,
DescribeOnlineListResponseData
>('/online/list', 'post', { ignoreError: true, });
