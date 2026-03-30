'use client';

import PositionList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function PositionsPage() {
  return (
    <Container>
      <PositionList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}
