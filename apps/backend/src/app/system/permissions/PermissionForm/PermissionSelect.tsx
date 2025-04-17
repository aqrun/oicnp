'use client';

import { useEffect, useMemo, useRef } from 'react';
import { Form, TreeSelect, TreeSelectProps } from 'antd';
import useFetchPermissionTree from './useFetchPermissionTree';
import { PermissionModel } from '@/services';

type TreeItem = NonNullable<TreeSelectProps['treeData']>[number];

/**
 * 权限选择器
 */
export default function PermissionSelect(): JSX.Element {
  const ref = useRef<HTMLDivElement>(null);
  const {
    treeData,
    loading,
    fetchTree,
  } = useFetchPermissionTree();

  const validTreeData = useMemo(() => {
    const list = treeData?.map((item) => {
      const node: TreeItem = {
        value: item.id,
        title: item.label,
      }

      if (item.children?.length) {
        node.children = item.children.map((child) => {
          const childNode: TreeItem = {
            value: child.id,
            title: child.label,
          }

          if (child.children?.length) {
            childNode.children = child.children.map((grandChild) => {
              return {
                value: grandChild.id,
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

  useEffect(() => {
    fetchTree();
  }, []);

  return (
    <div ref={ref}>
      <Form.Item<PermissionModel>
        label="父级权限"
        name="pid"
      >
        <TreeSelect
          showSearch
          placeholder="请选择权限"
          allowClear
          treeData={validTreeData}
          loading={loading}
          // treeDefaultExpandAll
          listHeight={200}
          getPopupContainer={() => ref.current || document.body}
        />
      </Form.Item>
    </div>
  )
}