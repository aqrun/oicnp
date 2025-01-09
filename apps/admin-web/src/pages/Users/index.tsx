import { useEffect } from 'react';
import { fetchMenus } from '~/api';
import { Table } from 'antd';
import {
  PageTitle,
  Filters,
} from '~/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';
import { FilterValues } from '~/types';
import useColumns from './useColumns';
import { useQuery } from '@tanstack/react-query';
import { fetchUserList, UserListData } from '~/api';

/**
 * 仪表盘
 */
export default function Users(): JSX.Element {
  const columns = useColumns();

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['userList'],
    queryFn: async () => {
      const res = await fetchUserList();
      return res?.data;
    }
  });

  const getDataSource = () => {
    return data || [];
  };
  const dataSource = getDataSource();

  const handleCreate = useMemoizedFn(() => {

  });

  const handleRefresh = useMemoizedFn(() => {
    refetch();
  });

  const handleSearch = useMemoizedFn((values: FilterValues) => {

  });


  useEffect(() => {
    console.log('user--mount----');
    fetchMenus();
  }, []);

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
      />
      
      <Table<UserListData>
        dataSource={dataSource}
        columns={columns}
        loading={isFetching}
        rowKey="id"
        size="small"
        tableLayout="fixed"
      />
    </Container>
  );
}