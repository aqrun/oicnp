import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const user = useViewStore(state => state.user);

  const items: DescriptionsProps['items'] = [
    {
      key: 'uid',
      label: 'ID',
      children: user?.uid,
    },
    {
      key: 'nickname',
      label: '名称',
      children: user?.nickname,
    },
    {
      key: 'uuid',
      label: 'VID',
      children: user?.uuid,
    },
    {
      key: 'phone',
      label: '电话',
      children: user?.phone,
    },
    {
      key: 'email',
      label: '邮箱',
      children: user?.email,
    },
    {
      key: 'status',
      label: '状态',
      children: user?.status === '1' ? '启用' : '禁用',
    },
    {
      key: 'roles',
      label: '角色',
      children: user?.roles?.map(item => item).join(','),
    },
    {
      key: 'dptId',
      label: '部门',
      children: user?.dptId,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: user?.createdAt ? formatDate(user?.createdAt) : '-',
    },
    {
      key: 'updatedAt',
      label: '更新时间',
      children: user?.updatedAt ? formatDate(user?.updatedAt) : '-',
    },
  ];

  return [items];
}
