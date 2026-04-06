import type { CreateService } from "@repo/services";
import {
  DescribePoetryListWithChaptersRequestParams,
  DescribePoetryListWithChaptersResponseData,
} from "./types";

export function createPoetryApis(createService: CreateService) {
  return {
    DescribePoetryListWithChapters: createService<
      DescribePoetryListWithChaptersRequestParams,
      DescribePoetryListWithChaptersResponseData
    >("poetry/list-with-chapters", "post", { ignoreError: true }),
  };
}
