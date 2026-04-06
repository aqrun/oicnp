'use client';

import type { ReactElement } from "react";

import { useEffect } from 'react';
import { Table } from 'antd';
import {   PageTitle, Filters, } from '#src/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '#src/types';
import useColumns from './useColumns';
import { useListStore } from './useListStore';
import { nextTick } from '#src/utils';
import { useList } from './useList';
import { LoginLogModel } from "@repo/apis";
import { BasicPage } from "#src/components/basic-page";
import { Container } from './index.styled';
/**
 * 登录日志列表
 */
export default function PositionList(): ReactElement {
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const columns = useColumns();

  const {
    listRes,
    loading,
    refresh,
    fetchListPageData,
  } = useList();

  const getDataSource = () => {
    const list = listRes?.logs || [];
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

    return () => {
      setState({
        listRes: undefined,
        pager: {
          page: 1,
          pageSize: 10,
          total: 0,
        },
        filters: {},
      });
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
 
  return (
    <BasicPage>
      <Container>
      <PageTitle
        title='登录日志'
      />
      <Filters
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
      />
      
      <Table<LoginLogModel>
        dataSource={dataSource}
        columns={columns}
        loading={loading}
        rowKey="id"
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