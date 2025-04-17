import { useState } from 'react';
import {
  PermissionTreeItem,
  DescribePermissionTree,
} from '@/services';
import { useMemoizedFn } from 'ahooks';

export default function useFetchPermissionTree() {
  const [treeData, setTreeData] = useState<PermissionTreeItem[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchTree = useMemoizedFn(async () => {
    setLoading(true);
    const res = await DescribePermissionTree({});
    setTreeData(res.permissions);
    setLoading(false);
    return res;
  });

  return {
    treeData,
    fetchTree,
    loading,
  }
}