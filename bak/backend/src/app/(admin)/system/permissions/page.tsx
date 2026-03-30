'use client';

import PermissionList from './PermissionList';
import CreateModal from './create/CreateModal';
import ViewModal from './view/ViewModal';
import EditModal from './edit/EditModal';

export default function PermissionPage() {

  return (
    <>
      <PermissionList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </>
  );
}
