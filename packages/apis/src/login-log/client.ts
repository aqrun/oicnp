import type { CreateService } from "@repo/services";
import {
  DescribeLoginLogListRequestParams,
  DescribeLoginLogListResponseData,
  DescribeLoginLogDetailRequestParams,
  DescribeLoginLogDetailResponseData,
  DescribeDeleteLoginLogRequestParams,
  DescribeDeleteLoginLogResponseData,
  DescribeCreateLoginLogRequestParams,
  DescribeCreateLoginLogResponseData,
  DescribeUpdateLoginLogRequestParams,
  DescribeUpdateLoginLogResponseData,
} from "./types";

export function createLoginLogApis(createService: CreateService) {
  return {
    DescribeLoginLogList: createService<
      DescribeLoginLogListRequestParams,
      DescribeLoginLogListResponseData
    >("login-log/list", "post", { ignoreError: true }),
    DescribeLoginLogDetail: createService<
      DescribeLoginLogDetailRequestParams,
      DescribeLoginLogDetailResponseData
    >("login-log/one", "post", { ignoreError: true }),
    DescribeDeleteLoginLog: createService<
      DescribeDeleteLoginLogRequestParams,
      DescribeDeleteLoginLogResponseData
    >("login-log/remove", "post"),
    DescribeCreateLoginLog: createService<
      DescribeCreateLoginLogRequestParams,
      DescribeCreateLoginLogResponseData
    >("login-log/add", "post"),
    DescribeUpdateLoginLog: createService<
      DescribeUpdateLoginLogRequestParams,
      DescribeUpdateLoginLogResponseData
    >("login-log/update", "post"),
  };
}
