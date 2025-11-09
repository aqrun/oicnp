'use client';
import SettingsLayout from './SettingsLayout';

export default function Layout({
  children,
}: React.PropsWithChildren): JSX.Element {
  return (
    <SettingsLayout>
      {children}
    </SettingsLayout>
  );
}
