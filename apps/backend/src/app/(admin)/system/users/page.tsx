'use client';

import UserList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function DepartmentsPage() {
  return (
    <Container>
      <UserList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}
