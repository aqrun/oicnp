'use client';

import OperationLogList from './List';
import ViewModal from './detail/ViewModal';
import { Container } from './index.styled';

export default function OperationLogsPage() {
  return (
    <Container>
      <OperationLogList />
      <ViewModal />
    </Container>
  );
}
