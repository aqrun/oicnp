import type { CreateService } from "@repo/services";
import {
  DescribePositionListRequestParams,
  DescribePositionListResponseData,
  DescribePositionDetailRequestParams,
  DescribePositionDetailResponseData,
  DescribeDeletePositionRequestParams,
  DescribeDeletePositionResponseData,
  DescribeCreatePositionRequestParams,
  DescribeCreatePositionResponseData,
  DescribeUpdatePositionRequestParams,
  DescribeUpdatePositionResponseData,
} from "./types";

export function createPositionApis(createService: CreateService) {
  return {
    DescribePositionList: createService<
      DescribePositionListRequestParams,
      DescribePositionListResponseData
    >("position/list", "post", { ignoreError: true }),
    DescribePositionDetail: createService<
      DescribePositionDetailRequestParams,
      DescribePositionDetailResponseData
    >("position/one", "post", { ignoreError: true }),
    DescribeDeletePosition: createService<
      DescribeDeletePositionRequestParams,
      DescribeDeletePositionResponseData
    >("position/remove", "post"),
    DescribeCreatePosition: createService<
      DescribeCreatePositionRequestParams,
      DescribeCreatePositionResponseData
    >("position/add", "post"),
    DescribeUpdatePosition: createService<
      DescribeUpdatePositionRequestParams,
      DescribeUpdatePositionResponseData
    >("position/update", "post"),
  };
}
