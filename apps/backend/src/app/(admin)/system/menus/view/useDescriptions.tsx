import { useMemo } from 'react';
import { DescriptionsProps, Tag } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';
import { Icon } from '@/components';
export default function useDescriptions() {
  const menu = useViewStore(state => state.menu);
  const menuPermissions = useViewStore(state => state.menuPermissions);

  const permissionTags = useMemo(() => {
    if (!menuPermissions?.length) {
      return '-';
    }

    return menuPermissions?.map((item) => {
      return <Tag key={item.permissionId}>{item.name}</Tag>;
    });
  }, [menuPermissions]);

  const items: DescriptionsProps['items'] = [
    {
      key: 'vid',
      label: '标识',
      children: menu?.vid,
    },
    {
      key: 'name',
      label: '名称',
      children: menu?.name,
    },
    {
      key: 'icon',
      label: '图标',
      children: <Icon icon={menu?.icon || ''} />,
    },
    {
      key: 'status',
      label: '状态',
      children: menu?.status === '1' ? '启用' : '禁用',
    },
    {
      key: 'weight',
      label: '排序',
      children: menu?.weight,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: menu?.createdAt ? formatDate(menu?.createdAt) : '-',
      span: 12,
    },
    {
      key: 'remark',
      label: '描述',
      children: menu?.remark || '-',
      span: 24,
    },
    {
      key: 'permissions',
      label: '权限列表',
      children: permissionTags,
      span: 12,
    },
  ];

  return [items];
}
