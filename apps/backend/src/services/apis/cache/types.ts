import {
  BaseFilterParams,
  BaseListResponseData,
} from '../../types';

export interface CacheModel {
  id?: number;
  cacheKey?: string;
  cacheValue?: string;
  scope?: string;
  createdAt?: string;
  expiredAt?: string;
}

export interface CacheScopeModel {
  scope: string;
  label: string;
}

export interface CacheFilters {
  id?: number;
  cacheKey?: string;
  cacheValue?: string;
  scope?: string;
  createdAt?: string;
  expiredAt?: string;
}

export interface DescribeCacheDetailRequestParams extends CacheFilters {
  _name?: string;
}
export interface DescribeCacheDetailResponseData {
  cache: CacheModel;
}

export interface DescribeCacheListRequestParams extends BaseFilterParams {
  scope?: string;
  _name?: string;
}

export interface DescribeCacheListResponseData extends BaseListResponseData {
  caches: Array<CacheModel>;
}

export interface DescribeCacheScopeListRequestParams {
  _name?: string;
}

export interface DescribeCacheScopeListResponseData {
  scopes: Array<CacheScopeModel>;
}
