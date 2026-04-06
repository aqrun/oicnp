import type { CreateService } from "@repo/services";
import {
  DescribeCronListRequestParams,
  DescribeCronListResponseData,
} from "./types";

export function createCronApis(createService: CreateService) {
  return {
    DescribeCronList: createService<
      DescribeCronListRequestParams,
      DescribeCronListResponseData
    >("cron/list", "post", { ignoreError: true }),
  };
}
