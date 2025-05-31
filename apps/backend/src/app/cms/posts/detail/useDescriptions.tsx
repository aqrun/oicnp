import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const node = useViewStore(state => state.node);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: node?.nid,
    },
    {
      key: 'uuid',
      label: 'UUID',
      children: node?.uuid,
    },
    {
      key: 'vid',
      label: '标识',
      children: node?.vid,
    },
    {
      key: 'title',
      label: '标题',
      children: node?.title,
    },
    {
      key: 'viewed',
      label: '浏览量',
      children: node?.viewed,
    },
    {
      key: 'publishedAt',
      label: '发布时间',
      children: node?.publishedAt,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: formatDate(node?.createdAt || ''),
    }
  ];

  return [items];
}
