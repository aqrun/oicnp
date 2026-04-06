import type { CreateService } from "@repo/services";
import {
  DescribeCacheListRequestParams,
  DescribeCacheListResponseData,
  DescribeCacheDetailRequestParams,
  DescribeCacheDetailResponseData,
  DescribeCacheScopeListRequestParams,
  DescribeCacheScopeListResponseData,
} from "./types";

export function createCacheApis(createService: CreateService) {
  return {
    DescribeCacheList: createService<
      DescribeCacheListRequestParams,
      DescribeCacheListResponseData
    >("cache/list", "post", { ignoreError: true }),
    DescribeCacheDetail: createService<
      DescribeCacheDetailRequestParams,
      DescribeCacheDetailResponseData
    >("cache/one", "post", { ignoreError: true }),
    DescribeCacheScopeList: createService<
      DescribeCacheScopeListRequestParams,
      DescribeCacheScopeListResponseData
    >("cache/scope-list", "post", { ignoreError: true }),
  };
}
