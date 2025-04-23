'use client';

import { useMemo, useRef } from 'react';
import { Tree, TreeProps, Spin } from 'antd';
import usePermissionTree from './usePermissionTree';
import {
  PermissionTreeWrapper,
} from './index.styled';

type TreeItem = NonNullable<TreeProps['treeData']>[number];

export interface PermissionTreeProps {
  onCheck?: TreeProps['onCheck'];
  defaultCheckedKeys?: Array<React.Key>;
}

/**
 * 权限树选择器
 */
export default function PermissionTree({
  onCheck,
  defaultCheckedKeys,
}: PermissionTreeProps): JSX.Element {
  const ref = useRef<HTMLDivElement>(null);

  const {
    treeData,
    loading,
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
    <PermissionTreeWrapper ref={ref} className="rounded-sm border border-gray-200 p-2">
      <Tree
        checkable
        treeData={validTreeData}
        onCheck={onCheck}
        defaultExpandParent
        defaultExpandedKeys={defaultExpandedKeys}
        defaultCheckedKeys={defaultCheckedKeys}
      />
    </PermissionTreeWrapper>
  )
}
