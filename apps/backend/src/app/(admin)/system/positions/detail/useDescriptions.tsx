import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const position = useViewStore(state => state.position);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: position?.positionId,
    },
    {
      key: 'name',
      label: '名称',
      children: position?.name,
    },
    {
      key: 'vid',
      label: 'VID',
      children: position?.vid,
    },
    {
      key: 'weight',
      label: '权重',
      children: position?.weight,
    },
    {
      key: 'remark',
      label: '备注',
      children: position?.remark,
    },
    {
      key: 'status',
      label: '状态',
      children: position?.status,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: position?.createdAt ? formatDate(position?.createdAt) : '-',
    },
    {
      key: 'updatedAt',
      label: '更新时间',
      children: position?.updatedAt ? formatDate(position?.updatedAt) : '-',
    },
  ];

  return [items];
}
