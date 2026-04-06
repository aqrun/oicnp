import type { CreateService } from "@repo/services";
import {
  DescribeNoteListRequestParams,
  DescribeNoteListResponseData,
  DescribeNoteDetailRequestParams,
  DescribeNoteDetailResponseData,
  DescribeDeleteNoteRequestParams,
  DescribeDeleteNoteResponseData,
  DescribeCreateNoteRequestParams,
  DescribeCreateNoteResponseData,
  DescribeUpdateNoteRequestParams,
  DescribeUpdateNoteResponseData,
} from './types';

export function createNoteApis(createService: CreateService) {
  return {
    DescribeNoteList: createService<
      DescribeNoteListRequestParams,
      DescribeNoteListResponseData
    >("note/list", "post", { ignoreError: true }),
    DescribeNoteDetail: createService<
      DescribeNoteDetailRequestParams,
      DescribeNoteDetailResponseData
    >("note/one", "post", { ignoreError: true }),
    DescribeDeleteNote: createService<
      DescribeDeleteNoteRequestParams,
      DescribeDeleteNoteResponseData
    >("note/remove", "post"),
    DescribeCreateNote: createService<
      DescribeCreateNoteRequestParams,
      DescribeCreateNoteResponseData
    >("note/add", "post"),
    DescribeUpdateNote: createService<
      DescribeUpdateNoteRequestParams,
      DescribeUpdateNoteResponseData
    >("note/update", "post"),
  };
}
