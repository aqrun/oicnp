import type { CreateService } from "@repo/services";
import {
  DescribeNodeListRequestParams,
  DescribeNodeListResponseData,
  DescribeNodeDetailRequestParams,
  DescribeNodeDetailResponseData,
  DescribeDeleteNodeRequestParams,
  DescribeDeleteNodeResponseData,
  DescribeCreateNodeRequestParams,
  DescribeCreateNodeResponseData,
  DescribeUpdateNodeRequestParams,
  DescribeUpdateNodeResponseData,
  DescribeNodeTagsRequestParams,
  DescribeNodeTagsResponseData,
  DescribeNodeCategoriesRequestParams,
  DescribeNodeCategoriesResponseData,
  DescribeNodeBodyRequestParams,
  DescribeNodeBodyResponseData,
} from "./types";

export function createNodeApis(createService: CreateService) {
  return {
    DescribeNodeList: createService<
      DescribeNodeListRequestParams,
      DescribeNodeListResponseData
    >("node/list", "post", { ignoreError: true }),
    DescribeNodeDetail: createService<
      DescribeNodeDetailRequestParams,
      DescribeNodeDetailResponseData
    >("node/one", "post", { ignoreError: true }),
    DescribeDeleteNode: createService<
      DescribeDeleteNodeRequestParams,
      DescribeDeleteNodeResponseData
    >("node/remove", "post"),
    DescribeCreateNode: createService<
      DescribeCreateNodeRequestParams,
      DescribeCreateNodeResponseData
    >("node/add", "post"),
    DescribeUpdateNode: createService<
      DescribeUpdateNodeRequestParams,
      DescribeUpdateNodeResponseData
    >("node/update", "post"),
    DescribeNodeTags: createService<
      DescribeNodeTagsRequestParams,
      DescribeNodeTagsResponseData
    >("node/tags", "post"),
    DescribeNodeCategories: createService<
      DescribeNodeCategoriesRequestParams,
      DescribeNodeCategoriesResponseData
    >("node/categories", "post"),
    DescribeNodeBody: createService<
      DescribeNodeBodyRequestParams,
      DescribeNodeBodyResponseData
    >("node/body", "post"),
  };
}
