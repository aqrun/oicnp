import type { CreateService } from "@repo/services";
import {
  DescribeMenuListRequestParams,
  DescribeMenuListResponseData,
  DescribeMenuDetailRequestParams,
  DescribeMenuDetailResponseData,
  DescribeDeleteMenuRequestParams,
  DescribeDeleteMenuResponseData,
  DescribeCreateMenuRequestParams,
  DescribeCreateMenuResponseData,
  DescribeUpdateMenuRequestParams,
  DescribeUpdateMenuResponseData,
  DescribeMenuPermissionsResponseData,
  DescribeMenuPermissionsRequestParams,
  DescribeMenuTreeRequestParams,
  DescribeMenuTreeResponseData,
} from "./types";

export function createMenuApis(createService: CreateService) {
  return {
    DescribeMenuList: createService<
      DescribeMenuListRequestParams,
      DescribeMenuListResponseData
    >("menu/list", "post", { ignoreError: true }),
    DescribeMenuDetail: createService<
      DescribeMenuDetailRequestParams,
      DescribeMenuDetailResponseData
    >("menu/one", "post", { ignoreError: true }),
    DescribeDeleteMenu: createService<
      DescribeDeleteMenuRequestParams,
      DescribeDeleteMenuResponseData
    >("menu/remove", "post"),
    DescribeCreateMenu: createService<
      DescribeCreateMenuRequestParams,
      DescribeCreateMenuResponseData
    >("menu/add", "post"),
    DescribeUpdateMenu: createService<
      DescribeUpdateMenuRequestParams,
      DescribeUpdateMenuResponseData
    >("menu/update", "post"),
    DescribeMenuPermissions: createService<
      DescribeMenuPermissionsRequestParams,
      DescribeMenuPermissionsResponseData
    >("menu/permissions", "post"),
    DescribeMenuTree: createService<
      DescribeMenuTreeRequestParams,
      DescribeMenuTreeResponseData
    >("menu/tree", "post"),
  };
}
