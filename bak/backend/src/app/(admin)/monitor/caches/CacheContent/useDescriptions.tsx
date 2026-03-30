import { DescriptionsProps } from 'antd';
import { useListStore } from '../CacheList/useListStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const cacheDetailRes = useListStore(state => state.cacheDetailRes);
  const cache = cacheDetailRes?.cache;

  let cacheValue = cache?.cacheValue;

  try {
    cacheValue = JSON.parse(cache?.cacheValue || '{}');
  } catch (err) {
    console.log('err', err);
  }

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
      children: (
        <pre>
          <code>
            {JSON.stringify(cacheValue, null, 2)}
          </code>
        </pre>
      ),
    },
    {
      key: 'scope',
      label: '缓存分类',
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
