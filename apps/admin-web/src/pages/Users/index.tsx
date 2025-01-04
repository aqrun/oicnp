import { useEffect } from 'react';
import { fetchMenus } from '~/api';
import { Container } from './index.styled';

/**
 * 仪表盘
 */
export default function Users(): JSX.Element {
  useEffect(() => {
    console.log('user--mount----');
    fetchMenus();
  }, []);

  return (
    <Container>
      users
    </Container>
  );
}