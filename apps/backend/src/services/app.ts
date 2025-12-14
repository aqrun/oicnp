'use client';

import { createService } from './request';
import {
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
} from './apis/user/types';

export const DescribeLogin = createService<
DescribeLoginRequestParams,
DescribeLoginResponseData
>('/auth/login', 'post');
