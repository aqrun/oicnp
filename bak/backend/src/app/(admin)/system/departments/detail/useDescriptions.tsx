import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const department = useViewStore(state => state.department);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: department?.id,
    },
    {
      key: 'name',
      label: '名称',
      children: department?.name,
    },
    {
      key: 'vid',
      label: 'VID',
      children: department?.vid,
    },
    {
      key: 'weight',
      label: '权重',
      children: department?.weight,
    },
    {
      key: 'leader',
      label: '负责人',
      children: department?.leader,
    },
    {
      key: 'phone',
      label: '电话',
      children: department?.phone,
    },
    {
      key: 'email',
      label: '邮箱',
      children: department?.email,
    },
    {
      key: 'status',
      label: '状态',
      children: department?.status,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: department?.createdAt ? formatDate(department?.createdAt) : '-',
    },
    {
      key: 'updatedAt',
      label: '更新时间',
      children: department?.updatedAt ? formatDate(department?.updatedAt) : '-',
    },
  ];

  return [items];
}
