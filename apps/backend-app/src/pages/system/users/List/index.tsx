'use client';

import type { ReactElement } from "react";

import { useEffect } from 'react';
import { Table } from 'antd';
import {
  PageTitle,
  Filters,
} from '#src/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '#src/types';
import useColumns from './useColumns';
import { useListStore } from './useListStore';
import { useCreateStore } from '../create/useCreateStore';
import { nextTick } from '#src/utils';
import { useList } from './useList';
import type { UserModel } from '@repo/apis';
import { BasicPage } from "#src/components/basic-page";
import { Container } from './index.styled';
/**
 * 用户列表
 */
export default function UserList(): ReactElement {
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setCreateState = useCreateStore((state) => state.setState);
  const columns = useColumns();

  const {
    listRes,
    loading,
    refresh,
    fetchListPageData,
  } = useList();

  const getDataSource = () => {
    const list = listRes?.users || [];
    return list;
  };
  const dataSource = getDataSource();

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
   * 创建操作
   */
  const handleCreate = useMemoizedFn(() => {
    setCreateState({
      visible: true,
    });
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
      fetchListPageData();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshToken]);
  useEffect(() => {
    refresh();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
 
  return (
    <BasicPage>
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
          scroll={{
            x: 'max-content',
          }}
          pagination={{
            total: pager?.total,
            pageSize: pager?.pageSize,
            showQuickJumper: true,
            onChange: handlePagerChange,
          }}
        />
      </Container>
    </BasicPage>
  );
}