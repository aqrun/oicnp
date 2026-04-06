import type { CreateService } from "@repo/services";
import {
  DescribePermissionListRequestParams,
  DescribePermissionListResponseData,
  DescribePermissionDetailRequestParams,
  DescribePermissionDetailResponseData,
  DescribeDeletePermissionRequestParams,
  DescribeDeletePermissionResponseData,
  DescribeCreatePermissionRequestParams,
  DescribeCreatePermissionResponseData,
  DescribeUpdatePermissionRequestParams,
  DescribeUpdatePermissionResponseData,
  DescribePermissionTreeRequestParams,
  DescribePermissionTreeResponseData,
} from "./types";

export function createPermissionApis(createService: CreateService) {
  return {
    DescribePermissionList: createService<
      DescribePermissionListRequestParams,
      DescribePermissionListResponseData
    >("permission/list", "post", { ignoreError: true }),
    DescribePermissionTree: createService<
      DescribePermissionTreeRequestParams,
      DescribePermissionTreeResponseData
    >("permission/tree", "post", { ignoreError: true }),
    DescribePermissionDetail: createService<
      DescribePermissionDetailRequestParams,
      DescribePermissionDetailResponseData
    >("permission/one", "post", { ignoreError: true }),
    DescribeDeletePermission: createService<
      DescribeDeletePermissionRequestParams,
      DescribeDeletePermissionResponseData
    >("permission/remove", "post"),
    DescribeCreatePermission: createService<
      DescribeCreatePermissionRequestParams,
      DescribeCreatePermissionResponseData
    >("permission/add", "post"),
    DescribeUpdatePermission: createService<
      DescribeUpdatePermissionRequestParams,
      DescribeUpdatePermissionResponseData
    >("permission/update", "post"),
  };
}
