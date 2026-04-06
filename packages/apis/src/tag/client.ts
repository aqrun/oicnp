import type { CreateService } from "@repo/services";
import {
  DescribeTagListRequestParams,
  DescribeTagListResponseData,
  DescribeTagDetailRequestParams,
  DescribeTagDetailResponseData,
  DescribeDeleteTagRequestParams,
  DescribeDeleteTagResponseData,
  DescribeCreateTagRequestParams,
  DescribeCreateTagResponseData,
  DescribeUpdateTagRequestParams,
  DescribeUpdateTagResponseData,
} from "./types";

export function createTagApis(createService: CreateService) {
  return {
    DescribeTagList: createService<
      DescribeTagListRequestParams,
      DescribeTagListResponseData
    >("tag/list", "post", { ignoreError: true }),
    DescribeTagDetail: createService<
      DescribeTagDetailRequestParams,
      DescribeTagDetailResponseData
    >("tag/one", "post", { ignoreError: true }),
    DescribeDeleteTag: createService<
      DescribeDeleteTagRequestParams,
      DescribeDeleteTagResponseData
    >("tag/remove", "post"),
    DescribeCreateTag: createService<
      DescribeCreateTagRequestParams,
      DescribeCreateTagResponseData
    >("tag/add", "post"),
    DescribeUpdateTag: createService<
      DescribeUpdateTagRequestParams,
      DescribeUpdateTagResponseData
    >("tag/update", "post"),
  };
}
