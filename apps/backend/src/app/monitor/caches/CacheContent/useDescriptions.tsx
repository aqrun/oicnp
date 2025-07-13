import { DescriptionsProps } from 'antd';
import { useListStore } from '../CacheList/useListStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const cacheDetailRes = useListStore(state => state.cacheDetailRes);
  const cache = cacheDetailRes?.cache;

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: cache?.id,
    },
    {
      key: 'cacheKey',
      label: '缓存键名',
      children: cache?.cacheKey,
    },
    {
      key: 'cacheValue',
      label: '缓存值',
      children: cache?.cacheValue,
    },
    {
      key: 'scope',
      label: '缓存范围',
      children: cache?.scope,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: cache?.createdAt ? formatDate(cache?.createdAt) : '-',
    },
    {
      key: 'expiredAt',
      label: '过期时间',
      children: cache?.expiredAt ? formatDate(cache?.expiredAt) : '-',
    },
  ];

  return [items];
}
