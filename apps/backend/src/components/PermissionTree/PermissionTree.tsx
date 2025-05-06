'use client';

import { useMemo, useRef } from 'react';
import { Tree, TreeProps, Spin } from 'antd';
import { useMemoizedFn } from 'ahooks';
import usePermissionTree from './usePermissionTree';
import { callFn } from '@/utils';
import {
  PermissionTreeWrapper,
} from './index.styled';

type TreeItem = NonNullable<TreeProps['treeData']>[number];

export interface PermissionTreeProps {
  onCheck?: TreeProps['onCheck'];
  checkedKeys?: Array<React.Key>;
  onCheckChange?: (keys: Array<React.Key>) => void;
  multiple?: boolean;
}

/**
 * 权限树选择器
 */
export default function PermissionTree({
  onCheck,
  checkedKeys: paramCheckedKeys,
  onCheckChange,
  multiple = true,
}: PermissionTreeProps): JSX.Element {
  const ref = useRef<HTMLDivElement>(null);

  const {
    treeData,
    loading,
    checkedKeys,
    setState,
  } = usePermissionTree();

  const validTreeData = useMemo(() => {
    const list = treeData?.map((item) => {
      const node: TreeItem = {
        key: item.id,
        title: item.label,
      }

      if (item.children?.length) {
        node.children = item.children.map((child) => {
          const childNode: TreeItem = {
            key: child.id,
            title: child.label,
          }

          if (child.children?.length) {
            childNode.children = child.children.map((grandChild) => {
              return {
                key: grandChild.id,
                title: grandChild.label,
              }
            });
          }
          return childNode;
        });
      }

      return node;
    });

    return list;
  }, [treeData]);

  const handleCheck: TreeProps['onCheck'] = useMemoizedFn((keys: Array<React.Key>) => {
    setState({
      checkedKeys: keys,
    });

    callFn(onCheckChange, keys);
  }) as any;

  const defaultExpandedKeys = useMemo(() => {
    const list = treeData?.map((item) => {
      return item.id;
    });

    return list;
  }, [treeData]);

  if (loading) {
    return <Spin />;
  }

  return (
    <PermissionTreeWrapper ref={ref} className="rounded-sm border border-gray-200 p-2 w-full">
      <Tree
        checkable
        treeData={validTreeData}
        onCheck={onCheck || handleCheck}
        defaultExpandParent
        defaultExpandedKeys={defaultExpandedKeys}
        checkedKeys={paramCheckedKeys || checkedKeys}
        multiple={multiple}
      />
    </PermissionTreeWrapper>
  )
}
