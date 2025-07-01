'use client';

import { createService } from '../../request';
import {
  DescribeCaptchaRequestParams,
  DescribeCaptchaResponseData,
} from './types';

export const DescribeCaptcha = createService<
DescribeCaptchaRequestParams,
DescribeCaptchaResponseData
>('/captcha', 'get', { ignoreError: true, });
