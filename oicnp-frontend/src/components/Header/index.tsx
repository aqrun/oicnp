import React from 'react';
import dynamic from 'next/dynamic';

const HeaderBase = dynamic(
  () => import('./HeaderBase'),
  { ssr: false, }
);

export interface HeaderProps {
  menuId: number;
  activeVid?: string;
}

export const Header: React.FC<HeaderProps> = (props) => {
  return (
    <HeaderBase {...props} />
  );
}