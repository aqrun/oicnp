'use client';

import { useEffect } from 'react';
import { Table } from 'antd';
import {
  PageTitle,
  Filters,
} from '@/components';
import { useMemoizedFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '@/types';
import useColumns from './useColumns';
import { useListStore } from './useListStore';
import { nextTick } from '@/utils';
import { useQueryCategoryList } from './useQueryCategoryList';
import { useCreateStore } from '../create/useCreateStore';
import { CategoryModel } from '@/services';
import { Container } from './index.styled';
/**
 * 分类列表
 */
export default function CategoryList(): JSX.Element {
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setCreateState = useCreateStore(state => state.setState);
  const columns = useColumns();

  const {data, loading, refresh} = useQueryCategoryList();

  const getDataSource = () => {
    const list = data?.categories || [];
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
    <Container>
      <PageTitle
        title='分类列表'
      />
      <Filters
        createLabel="创建分类"
        onCreate={handleCreate}
        onRefresh={handleRefresh}
        onSearch={handleSearch}
        onChange={handleFilterChange}
      />
      
      <Table<CategoryModel>
        dataSource={dataSource}
        columns={columns}
        loading={loading}
        rowKey="catId"
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
  );
}