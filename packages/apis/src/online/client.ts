import type { CreateService } from "@repo/services";
import {
  DescribeOnlineListRequestParams,
  DescribeOnlineListResponseData,
  DescribeForceLogoutRequestParams,
  DescribeForceLogoutResponseData,
} from "./types";

export function createOnlineApis(createService: CreateService) {
  return {
    DescribeOnlineList: createService<
      DescribeOnlineListRequestParams,
      DescribeOnlineListResponseData
    >("online/list", "post", { ignoreError: true }),
    DescribeForceLogout: createService<
      DescribeForceLogoutRequestParams,
      DescribeForceLogoutResponseData
    >("online/force_logout", "post"),
  };
}
