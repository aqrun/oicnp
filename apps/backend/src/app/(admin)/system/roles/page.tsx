'use client';

import RoleList from './RoleList';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';
import EditModal from './edit/EditModal';

export default function RolesPage() {

  return (
    <>
      <RoleList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </>
  );
}
