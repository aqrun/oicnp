'use client';

import RoleList from './RoleList';
import CreateModal from './create/CreateModal';
import ViewModal from './detail/ViewModal';

export default function RolesPage() {

  return (
    <>
      <RoleList />
      <CreateModal />
      <ViewModal />
    </>
  );
}
