'use client';

import { useEffect, useMemo, useState } from 'react';
import { Table } from 'antd';
import { useRouter } from 'next/navigation';
import {
  PageTitle,
  Filters,
} from '@/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '@/types';
import useColumns from './useColumns';
import { PermissionModel } from '@/services';
import { useListStore } from './useListStore';
import { useCreateStore } from '../create/useCreateStore';
import {
  nextTick, r,
  convertPermissionListToTree,
} from '@/utils';
import { useQueryPermissionList } from './useQueryPermissionList';
import { Container } from './index.styled';

/**
 * 权限列表
 */
export default function PermissionList(): JSX.Element {
  const router = useRouter();
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setCreateState = useCreateStore(state => state.setState);
  const columns = useColumns();

  /**
   * 展开收起状态
   */
  const [expand, setExpand] = useState(true);
   /**
   * 当前表格操作的展开项
   */
   const [tableExpandKeys, setTableExpandKeys] = useState<string[] | undefined>(undefined);

  const {data, loading, refresh} = useQueryPermissionList();

  const dataSource = useMemo(() => {
    return convertPermissionListToTree(data?.data || []);
  }, [data]);

  const expandedRowKeys = useMemo(() => {
    // 表格有操作优化显示表格数据
    if (typeof tableExpandKeys !== 'undefined') {
      return tableExpandKeys;
    }

    if (expand) {
      const ids: Array<number> = [];

      dataSource.forEach((m) => {
        ids.push(m.permissionId || 0);

        (m?.children || []).forEach((n) => {
          ids.push(n?.permissionId || 0);
        });
      });
      return ids;
    }
    return [];
  }, [expand, dataSource, tableExpandKeys]);

  /**
   * 创建操作
   */
  const handleCreate = useMemoizedFn(() => {
    // router.push(r('/system/permissions/create'));
    setCreateState({
      visible: true,
    });
  });

  const handleRefresh = useMemoizedFn(() => {
    refresh();
  });

  /**
   * 搜索处理
   */
  const handleSearch = useMemoizedFn(async (values: FilterValues) => {
    setState({
      filters: values,
    });
    await nextTick();
    refresh();
  });

  const handleFilterChange = useMemoizedFn(async (values: FilterValues, trigger?: EnumFilterTrigger) => {
    setState({
      filters: values,
    });

    await nextTick();

    if (trigger === 'keyword') {
      refresh();
    }
  });

  /**
   * 展开收起操作
   */
  const handleFilterExpand = useMemoizedFn(() => {
    setExpand(!expand);
  });

  /**
   * 单个数据展开收起操作
   */
  const handleTableExpandChange = useMemoizedFn((keys: readonly React.Key[]) => {
    setTableExpandKeys(keys as unknown as Array<string>);
  });

  /**
   * 页码数据变化
   */
  const handlePagerChange = useMemoizedFn(async (page: number, pageSize: number) => {
    setState({
      pager: {
        ...pager,
        page,
        pageSize,
      }
    });
    await nextTick();
    refresh();
  });

  useEffect(() => {
    if (refreshToken) {
      refresh();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshToken]);

  return (
    <Container>
      <PageTitle
        title='权限列表'
      />
      <Filters
        createLabel="创建权限"
        onCreate={handleCreate}
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
        onExpand={handleFilterExpand}
      />
      
      <Table<PermissionModel>
        dataSource={dataSource}
        columns={columns}
        loading={loading}
        rowKey="permissionId"
        size="small"
        tableLayout="fixed"
        pagination={false}
        expandable={{
          expandedRowKeys,
          onExpandedRowsChange: handleTableExpandChange,
        }}
      />
    </Container>
  );
}