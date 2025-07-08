'use client';

import { createService } from '../../request';
import {
  DescribeCronListRequestParams,
  DescribeCronListResponseData,
} from './types';

export const DescribeCronList = createService<
DescribeCronListRequestParams,
DescribeCronListResponseData
>('/cron/list', 'post', { ignoreError: true, });
