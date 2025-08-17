'use client';

import { createService } from '../../request';
import {
  DescribeOnlineListRequestParams,
  DescribeOnlineListResponseData,
  DescribeForceLogoutRequestParams,
  DescribeForceLogoutResponseData,
} from './types';

export const DescribeOnlineList = createService<
DescribeOnlineListRequestParams,
DescribeOnlineListResponseData
>('/online/list', 'post', { ignoreError: true, });

export const DescribeForceLogout = createService<
DescribeForceLogoutRequestParams,
DescribeForceLogoutResponseData
>('/online/force_logout', 'post');
