import { Outlet } from 'react-router';

export default function Layout(): JSX.Element {
  return (
    <div>
      <Outlet />
    </div>
  );
}