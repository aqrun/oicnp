import { Form, Descriptions, DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const role = useViewStore(state => state.role);

  const items: DescriptionsProps['items'] = [
    {
      key: 'vid',
      label: '标识',
      children: role?.vid,
    },
    {
      key: 'name',
      label: '名称',
      children: role?.name,
    },
    {
      key: 'status',
      label: '状态',
      children: role?.status === '1' ? '启用' : '禁用',
    },
    {
      key: 'weight',
      label: '排序',
      children: role?.weight,
    },
    {
      key: 'remark',
      label: '描述',
      children: role?.remark || '-',
    },
    {
      key: 'permissions',
      label: '权限列表',
      children: '-',
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: role?.createdAt ? formatDate(role?.createdAt) : '-',
    },
  ];

  return [items];
}
