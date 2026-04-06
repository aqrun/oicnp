'use client';

import type { ReactElement } from "react";

import { useEffect } from 'react';
import { Table } from 'antd';
import {   PageTitle, Filters, } from '#src/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '#src/types';
import useColumns from './useColumns';
import { useListStore } from './useListStore';
import { useCreateStore } from '../create/useCreateStore';
import { nextTick } from '#src/utils';
import { useList } from './useList';
import { UploadFileRes } from "@repo/apis";
import { BasicPage } from "#src/components/basic-page";
import { Container } from './index.styled';
/**
 * 文件列表
 */
export default function FileList(): ReactElement {
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setCreateState = useCreateStore((state) => state.setState);
  const columns = useColumns();

  const {
    filesRes,
    loading,
    refresh,
    fetchListPageData,
  } = useList();

  const getDataSource = () => {
    const list: UploadFileRes[] = filesRes;
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
      fetchListPageData();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshToken]);

  useEffect(() => {
    fetchListPageData();
  }, []);
 
  return (
    <BasicPage>
      <Container>
      <PageTitle
        title='文件列表'
      />
      <Filters
        createLabel="创建文件"
        onCreate={handleCreate}
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
      />
      
      <Table<UploadFileRes>
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