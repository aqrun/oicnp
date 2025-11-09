'use client';

import LoginLogList from './List';
import ViewModal from './detail/ViewModal';
import { Container } from './index.styled';

export default function LoginLogsPage() {
  return (
    <Container>
      <LoginLogList />
      <ViewModal />
    </Container>
  );
}
