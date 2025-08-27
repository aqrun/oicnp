'use client';

import DepartmentList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function DepartmentsPage() {
  return (
    <Container>
      <DepartmentList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}
