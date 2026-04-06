import type { CreateService } from "@repo/services";
import {
  DescribeOperationLogListRequestParams,
  DescribeOperationLogListResponseData,
  DescribeOperationLogDetailRequestParams,
  DescribeOperationLogDetailResponseData,
  DescribeDeleteOperationLogRequestParams,
  DescribeDeleteOperationLogResponseData,
  DescribeCreateOperationLogRequestParams,
  DescribeCreateOperationLogResponseData,
  DescribeUpdateOperationLogRequestParams,
  DescribeUpdateOperationLogResponseData,
} from "./types";

export function createOperationLogApis(createService: CreateService) {
  return {
    DescribeOperationLogList: createService<
      DescribeOperationLogListRequestParams,
      DescribeOperationLogListResponseData
    >("operation-log/list", "post", { ignoreError: true }),
    DescribeOperationLogDetail: createService<
      DescribeOperationLogDetailRequestParams,
      DescribeOperationLogDetailResponseData
    >("operation-log/one", "post", { ignoreError: true }),
    DescribeDeleteOperationLog: createService<
      DescribeDeleteOperationLogRequestParams,
      DescribeDeleteOperationLogResponseData
    >("operation-log/remove", "post"),
    DescribeCreateOperationLog: createService<
      DescribeCreateOperationLogRequestParams,
      DescribeCreateOperationLogResponseData
    >("operation-log/add", "post"),
    DescribeUpdateOperationLog: createService<
      DescribeUpdateOperationLogRequestParams,
      DescribeUpdateOperationLogResponseData
    >("operation-log/update", "post"),
  };
}
