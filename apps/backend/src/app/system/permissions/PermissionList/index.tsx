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
import { usePermissionStore } from './usePermissionStore';
import {
  nextTick, r,
  convertPermissionListToTree,
} from '@/utils';
import { useQueryRoleList } from './useQueryPermissionList';
import { Container } from './index.styled';

/**
 * 权限列表
 */
export default function PermissionList(): JSX.Element {
  const router = useRouter();
  const pager = usePermissionStore((state) => state.pager);
  const setState = usePermissionStore((state) => state.setState);
  const refreshToken = usePermissionStore((state) => state.refreshToken);
  const columns = useColumns();

  /**
   * 展开收起状态
   */
  const [expand, setExpand] = useState(true);

  const {data, loading, refresh} = useQueryRoleList();

  const dataSource = useMemo(() => {
    return convertPermissionListToTree(data?.data || []);
  }, [data]);

  const expandedRowKeys = useMemo(() => {
    if (expand) {
      const ids: Array<string> = [];

      dataSource.forEach((m) => {
        ids.push(m.permissionId);

        (m?.children || []).forEach((n) => {
          ids.push(n?.permissionId);
        });
      });
      return ids;
    }
    return [];
  }, [expand, dataSource]);

  /**
   * 创建操作
   */
  const handleCreate = useMemoizedFn(() => {
    router.push(r('/system/roles/create'));
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
        }}
      />
    </Container>
  );
}