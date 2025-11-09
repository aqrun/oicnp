'use client';

import FileList from './List';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';
import { Container } from './index.styled';

export default function FilesPage() {
  return (
    <Container>
      <FileList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </Container>
  );
}
