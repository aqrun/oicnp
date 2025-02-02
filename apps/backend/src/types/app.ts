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

/**
 * hash 参数
 */
export type HashState = {
  route?: string;
  subRoute?: string;
} & Record<string, string | number>;
