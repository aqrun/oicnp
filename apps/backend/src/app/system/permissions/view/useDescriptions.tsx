import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const permission = useViewStore(state => state.permission);
  const parent = useViewStore(state => state.parentPermission);

  const items: DescriptionsProps['items'] = [
    {
      key: 'pid',
      label: '父级权限',
      children: parent ? parent?.name : '-',
      span: 24,
    },
    {
      key: 'vid',
      label: '标识',
      children: permission?.vid,
    },
    {
      key: 'name',
      label: '名称',
      children: permission?.name,
    },
    {
      key: 'status',
      label: '状态',
      children: permission?.status === '1' ? '启用' : '禁用',
    },
    {
      key: 'weight',
      label: '排序',
      children: permission?.weight,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: permission?.createdAt ? formatDate(permission?.createdAt) : '-',
    },
    {
      key: 'createdAt-empty',
      label: null,
      children: null,
    },
    {
      key: 'remark',
      label: '描述',
      children: permission?.remark || '-',
      span: 24,
    },
  ];

  return [items];
}
