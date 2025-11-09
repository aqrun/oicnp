'use client';

import MenuList from './MenuList';
import CreateModal from './create/CreateModal';
import ViewModal from './view/ViewModal';
import EditModal from './edit/EditModal';

export default function MenusPage() {

  return (
    <>
      <MenuList />
      <CreateModal />
      <ViewModal />
      <EditModal />
    </>
  );
}
