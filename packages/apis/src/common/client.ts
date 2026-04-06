import type { CreateService } from "@repo/services";
import {
  DescribeCaptchaRequestParams,
  DescribeCaptchaResponseData,
} from "./types";

export function createCommonApis(createService: CreateService) {
  return {
    DescribeCaptcha: createService<
      DescribeCaptchaRequestParams,
      DescribeCaptchaResponseData
    >("captcha", "get", { ignoreError: true }),
  };
}
