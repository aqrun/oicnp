import { BaseResponse } from "@repo/services";

export interface AuthCaptcha {
  id: string;
  text: string;
  img: string;
}

export interface DescribeCaptchaRequestParams {
  id?: string;
}

export interface DescribeCaptchaResponseData extends BaseResponse {
  captcha: AuthCaptcha;
}
