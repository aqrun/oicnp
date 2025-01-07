import { useEffect } from 'react';
import { fetchMenus } from '~/api';
import {
  PageTitle,
  Filters,
} from '~/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

/**
 * 仪表盘
 */
export default function Users(): JSX.Element {
  const handleCreate = useMemoizedFn(() => {

  });

  const handleRefresh = useMemoizedFn(() => {

  });

  const handleSearch = useMemoizedFn((value: string) => {

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
      <section>
        users
      </section>
    </Container>
  );
}