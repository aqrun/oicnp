import type { CreateService } from "@repo/services";
import {
  DescribeCategoryListRequestParams,
  DescribeCategoryListResponseData,
  DescribeCategoryDetailRequestParams,
  DescribeCategoryDetailResponseData,
  DescribeDeleteCategoryRequestParams,
  DescribeDeleteCategoryResponseData,
  DescribeCreateCategoryRequestParams,
  DescribeCreateCategoryResponseData,
  DescribeUpdateCategoryRequestParams,
  DescribeUpdateCategoryResponseData,
} from "./types";

export function createCategoryApis(createService: CreateService) {
  return {
    DescribeCategoryList: createService<
      DescribeCategoryListRequestParams,
      DescribeCategoryListResponseData
    >("category/list", "post", { ignoreError: true }),
    DescribeCategoryDetail: createService<
      DescribeCategoryDetailRequestParams,
      DescribeCategoryDetailResponseData
    >("category/one", "post", { ignoreError: true }),
    DescribeDeleteCategory: createService<
      DescribeDeleteCategoryRequestParams,
      DescribeDeleteCategoryResponseData
    >("category/remove", "post"),
    DescribeCreateCategory: createService<
      DescribeCreateCategoryRequestParams,
      DescribeCreateCategoryResponseData
    >("category/add", "post"),
    DescribeUpdateCategory: createService<
      DescribeUpdateCategoryRequestParams,
      DescribeUpdateCategoryResponseData
    >("category/update", "post"),
  };
}
