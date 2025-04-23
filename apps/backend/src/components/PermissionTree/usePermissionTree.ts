import {
  PermissionTreeItem,
  DescribePermissionTree,
} from '@/services';
import { useMemoizedFn } from 'ahooks';
import { usePermissionTreeStore } from './usePermissionTreeStore';

/**
 * 权限树数据加载
 */
export default function usePermissionTree() {
  const treeData = usePermissionTreeStore(state => state.treeData);
  const loading = usePermissionTreeStore(state => state.loading);
  const setState = usePermissionTreeStore(state => state.setState);

  const fetchPermissionTree = useMemoizedFn(async () => {
    setState({
      loading: true,
    });
    const res = await DescribePermissionTree({});
  
    setState({
      treeData: res.permissions,
      loading: false,
    });
    return res;
  });

  return {
    treeData,
    fetchPermissionTree,
    loading,
  };
}