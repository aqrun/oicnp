import { get } from 'lodash';
import {
  ApolloClient,
  InMemoryCache,
} from "@apollo/client";
import { API_URL } from '~/constants';

export interface BaseResponse {
  [key: string]: any;
  page: number;
  pageSize: number;
  totalCount: number;
}

export const generateListResponseData = <T extends BaseResponse>(
  res: unknown,
  dataName: string
): T => {
  const list = get(res, `data.${dataName}.edges`, []);
  const dataList = list.map((item: any) => item?.node);
  const data = {
    [dataName]: dataList,
    page: get(res, 'data.nodes.page', 0),
    pageSize: get(res, 'data.nodes.pageSize', 0),
    totalCount: get(res, 'data.nodes.totalCount', 0),
  }
  return data as any;
}

export const client = new ApolloClient({
  uri: API_URL,
  cache: new InMemoryCache()
});