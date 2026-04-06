import type { CreateService } from "@repo/services";
import {
  DescribeDepartmentListRequestParams,
  DescribeDepartmentListResponseData,
  DescribeDepartmentDetailRequestParams,
  DescribeDepartmentDetailResponseData,
  DescribeDeleteDepartmentRequestParams,
  DescribeDeleteDepartmentResponseData,
  DescribeCreateDepartmentRequestParams,
  DescribeCreateDepartmentResponseData,
  DescribeUpdateDepartmentRequestParams,
  DescribeUpdateDepartmentResponseData,
} from "./types";

export function createDepartmentApis(createService: CreateService) {
  return {
    DescribeDepartmentList: createService<
      DescribeDepartmentListRequestParams,
      DescribeDepartmentListResponseData
    >("department/list", "post", { ignoreError: true }),
    DescribeDepartmentDetail: createService<
      DescribeDepartmentDetailRequestParams,
      DescribeDepartmentDetailResponseData
    >("department/one", "post", { ignoreError: true }),
    DescribeDeleteDepartment: createService<
      DescribeDeleteDepartmentRequestParams,
      DescribeDeleteDepartmentResponseData
    >("department/remove", "post"),
    DescribeCreateDepartment: createService<
      DescribeCreateDepartmentRequestParams,
      DescribeCreateDepartmentResponseData
    >("department/add", "post"),
    DescribeUpdateDepartment: createService<
      DescribeUpdateDepartmentRequestParams,
      DescribeUpdateDepartmentResponseData
    >("department/update", "post"),
  };
}
