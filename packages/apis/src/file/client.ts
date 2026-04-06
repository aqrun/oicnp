import type { CreateService } from "@repo/services";
import {
  DescribeFileListRequestParams,
  DescribeFileListResponseData,
  DescribeFileDetailRequestParams,
  DescribeFileDetailResponseData,
  DescribeDeleteFileRequestParams,
  DescribeDeleteFileResponseData,
  DescribeCreateFileRequestParams,
  DescribeCreateFileResponseData,
  DescribeUpdateFileRequestParams,
  DescribeUpdateFileResponseData,
} from "./types";

export function createFileApis(createService: CreateService) {
  return {
    DescribeFileList: createService<
      DescribeFileListRequestParams,
      DescribeFileListResponseData
    >("file/list", "post", { ignoreError: true }),
    DescribeFileDetail: createService<
      DescribeFileDetailRequestParams,
      DescribeFileDetailResponseData
    >("file/one", "post", { ignoreError: true }),
    DescribeDeleteFile: createService<
      DescribeDeleteFileRequestParams,
      DescribeDeleteFileResponseData
    >("file/remove", "post"),
    DescribeCreateFile: createService<
      DescribeCreateFileRequestParams,
      DescribeCreateFileResponseData
    >("file/add", "post"),
    DescribeUpdateFile: createService<
      DescribeUpdateFileRequestParams,
      DescribeUpdateFileResponseData
    >("file/update", "post"),
  };
}
