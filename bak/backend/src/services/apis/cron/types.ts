import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface BaseCronModel {
  id: number,
  vid: string,
  count: number,
  runCount: number,
  name: string,
  params: string,
  group: string,
  invokeTarget: string,
  expression: string,
  misfirePolicy: string,
  concurrent: string,
  status: string,
  remark: string,
  lastTime: string,
  nextTime: string,
  endTime: string,
  createdBy: number,
  updatedBy: number,
}

export interface CronModel extends BaseCronModel {
  createdAt?: string;
  updatedAt?: string;
}

export interface CronFieldType extends BaseCronModel {
  createdAt?: string;
}


export interface CronFilters {
  fileId?: number;
  uid?: string;
  filename?: string;
  uri?: string;
  storage?: string;
  mime?: string;
  status?: string;
}

export interface DescribeCronListRequestParams extends BaseFilterParams {
  _name?: string;
}

export interface DescribeCronListResponseData extends BaseListResponseData {
  crons: Array<CronModel>;
  _name?: string;
}
