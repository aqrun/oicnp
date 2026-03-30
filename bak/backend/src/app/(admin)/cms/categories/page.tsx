'use client';

import CategoryList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function SettingsPage() {
  return (
    <Container>
      <CategoryList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}
