'use client';

import UserList from './UserList';
import UserCreate from './UserCreate';
import { useGlobalState } from '@/context';

export default function UserPage() {
  const { hashState } = useGlobalState();
  console.log('hashState---', hashState)
  if (hashState?.route === 'create') {
    return (<UserCreate />);
  }

  return (
    <UserList />
  );
}
