import type { CreateService } from "@repo/services";
import {
  DescribeRoleListRequestParams,
  DescribeRoleListResponseData,
  DescribeRoleDetailRequestParams,
  DescribeRoleDetailResponseData,
  DescribeDeleteRoleRequestParams,
  DescribeDeleteRoleResponseData,
  DescribeCreateRoleRequestParams,
  DescribeCreateRoleResponseData,
  DescribeUpdateRoleRequestParams,
  DescribeUpdateRoleResponseData,
  DescribeRolePermissionsRequestParams,
  DescribeRolePermissionsResponseData,
} from "./types";

export function createRoleApis(createService: CreateService) {
  return {
    DescribeRoleList: createService<
      DescribeRoleListRequestParams,
      DescribeRoleListResponseData
    >("role/list", "post", { ignoreError: true }),
    DescribeRoleDetail: createService<
      DescribeRoleDetailRequestParams,
      DescribeRoleDetailResponseData
    >("role/one", "post", { ignoreError: true }),
    DescribeDeleteRole: createService<
      DescribeDeleteRoleRequestParams,
      DescribeDeleteRoleResponseData
    >("role/remove", "post"),
    DescribeCreateRole: createService<
      DescribeCreateRoleRequestParams,
      DescribeCreateRoleResponseData
    >("role/add", "post"),
    DescribeUpdateRole: createService<
      DescribeUpdateRoleRequestParams,
      DescribeUpdateRoleResponseData
    >("role/update", "post"),
    DescribeRolePermissions: createService<
      DescribeRolePermissionsRequestParams,
      DescribeRolePermissionsResponseData
    >("role/permissions", "post"),
  };
}
