'use client';

import NodeList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function SettingsPage() {
  return (
    <Container>
      <NodeList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}