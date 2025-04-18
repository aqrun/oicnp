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
import { UserModel } from '@/services';
import { useUserStore } from './useUserStore';
import { nextTick, r } from '@/utils';
import { useQueryUserList } from './useQueryUserList';
import { Container } from './index.styled';

/**
 * 仪表盘
 */
export default function UserList(): JSX.Element {
  const router = useRouter();
  const pager = useUserStore((state) => state.pager);
  const setState = useUserStore((state) => state.setState);
  const refreshToken = useUserStore((state) => state.refreshToken);
  const columns = useColumns();

  const {data, loading, refresh} = useQueryUserList();

  const getDataSource = () => {
    return data?.users || [];
  };
  const dataSource = getDataSource();

  /**
   * 创建操作
   */
  const handleCreate = useMemoizedFn(() => {
    router.push(r('/system/users/create'));
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
        title='用户列表'
      />
      <Filters
        createLabel="创建用户"
        onCreate={handleCreate}
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
      />
      
      <Table<UserModel>
        dataSource={dataSource}
        columns={columns}
        loading={loading}
        rowKey="uid"
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