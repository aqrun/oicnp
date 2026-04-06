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
import type { RoleModel } from '@repo/apis';
import { useListStore } from './useListStore';
import { nextTick } from '#src/utils';
import { useQueryRoleList } from './useQueryRoleList';
import { useCreateStore } from '../create/useCreateStore';
import { BasicPage } from "#src/components/basic-page";
import { Container } from './index.styled';

/**
 * 角色列表
 */
export default function RoleList(): ReactElement {
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setCreateState = useCreateStore(state => state.setState);
  const columns = useColumns();

  const {data, loading, refresh} = useQueryRoleList();

  const getDataSource = () => {
    const list = data?.roles || [];
    list.sort((a, b) => {
      const ia = a?.weight || 10000;
      const ib = b?.weight || 10000;
      return ia - ib;
    });
    return list;
  };
  const dataSource = getDataSource();

  /**
   * 创建操作
   */
  const handleCreate = useMemoizedFn(() => {
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
    <BasicPage>
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