import type { CreateService } from "@repo/services";
import {
  DescribeUserListRequestParams,
  DescribeUserListResponseData,
  DescribeUserDetailRequestParams,
  DescribeUserDetailResponseData,
  DescribeDeleteUserRequestParams,
  DescribeDeleteUserResponseData,
  DescribeCreateUserRequestParams,
  DescribeCreateUserResponseData,
  DescribeUpdateUserRequestParams,
  DescribeUpdateUserResponseData,
  DescribeUserRolesRequestParams,
  DescribeUserRolesResponseData,
  DescribeLoginRequestParams,
  DescribeLoginResponseData,
  DescribeAuthInfoRequestParams,
  DescribeAuthInfoResponseData,
} from "./types";

export function createUserApis(createService: CreateService) {
  return {
    DescribeUserList: createService<
      DescribeUserListRequestParams,
      DescribeUserListResponseData
    >("user/list", "post", { ignoreError: true }),
    DescribeUserDetail: createService<
      DescribeUserDetailRequestParams,
      DescribeUserDetailResponseData
    >("user/one", "post", { ignoreError: true }),
    DescribeDeleteUser: createService<
      DescribeDeleteUserRequestParams,
      DescribeDeleteUserResponseData
    >("user/remove", "post"),
    DescribeCreateUser: createService<
      DescribeCreateUserRequestParams,
      DescribeCreateUserResponseData
    >("user/add", "post"),
    DescribeUpdateUser: createService<
      DescribeUpdateUserRequestParams,
      DescribeUpdateUserResponseData
    >("user/update", "post"),
    DescribeUserRoles: createService<
      DescribeUserRolesRequestParams,
      DescribeUserRolesResponseData
    >("user/roles", "post"),
    DescribeAuthLogin: createService<
      DescribeLoginRequestParams,
      DescribeLoginResponseData
    >("auth/login", "post"),
    DescribeAuthInfo: createService<
      DescribeAuthInfoRequestParams,
      DescribeAuthInfoResponseData
    >("auth/info", "post"),
  };
}
