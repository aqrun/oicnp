'use client';

import { useEffect } from 'react';
import { Table } from 'antd';
import { useRouter } from 'next/navigation';
import {
  PageTitle,
  Filters,
} from '@/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '@/types';
import useColumns from './useColumns';
import { RoleModel } from '@/services';
import { useRoleStore } from './useRoleStore';
import { nextTick, r } from '@/utils';
import { useQueryRoleList } from './useQueryRoleList';
import { Container } from './index.styled';

/**
 * 角色列表
 */
export default function RoleList(): JSX.Element {
  const router = useRouter();
  const pager = useRoleStore((state) => state.pager);
  const setState = useRoleStore((state) => state.setState);
  const refreshToken = useRoleStore((state) => state.refreshToken);
  const columns = useColumns();

  const {data, loading, refresh} = useQueryRoleList();

  const getDataSource = () => {
    return data?.data || [];
  };
  const dataSource = getDataSource();console.log('dataSource---', dataSource);

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
        title='角色列表'
      />
      <Filters
        createLabel="创建角色"
        onCreate={handleCreate}
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
      />
      
      <Table<RoleModel>
        dataSource={dataSource}
        columns={columns}
        loading={loading}
        rowKey="roleId"
        size="small"
        tableLayout="fixed"
        pagination={{
          total: pager?.total,
          pageSize: pager?.pageSize,
          showQuickJumper: true,
          onChange: handlePagerChange,
        }}
      />
    </Container>
  );
}