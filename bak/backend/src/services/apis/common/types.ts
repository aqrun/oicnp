import { BaseResponse } from '../../types';

export interface AuthCaptcha {
  id: string;
  text: string;
  img: string;
}

export interface DescribeCaptchaRequestParams {
  id?: string;
}

export interface DescribeCaptchaResponseData {
  captcha: AuthCaptcha;
}

export interface ConsoleConfig {
  loginExpireTime: number;
  loginRememberExpireTime: number;
}

export interface DescribeConsoleConfigResponseData extends BaseResponse {
  config: ConsoleConfig;
}