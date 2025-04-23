import { useMemo } from 'react';
import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';
import {
  PermissionTree,
  usePermissionTree,
} from '@/components/PermissionTree';

export default function useDescriptions() {
  const role = useViewStore(state => state.role);
  const rolePermissions = useViewStore(state => state.rolePermissions);

  const {
    treeData,
  } = usePermissionTree();

  const permissionTreeData = useMemo(() => {
    
  }, [treeData, rolePermissions]);

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
      key: 'createdAt',
      label: '创建时间',
      children: role?.createdAt ? formatDate(role?.createdAt) : '-',
      span: 12,
    },
    {
      key: 'remark',
      label: '描述',
      children: role?.remark || '-',
      span: 24,
    },
    {
      key: 'permissions',
      label: '权限列表',
      children: (
        <PermissionTree

        />
      ),
      span: 12,
    },
  ];

  return [items];
}
