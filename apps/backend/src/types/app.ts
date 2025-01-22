/**
 * 筛选参数
 */
export interface FilterValues {
  keyword?: string;
}

export enum EnumFilterTrigger {
  /** 搜索框 */
  keyword = 'keyword',
}