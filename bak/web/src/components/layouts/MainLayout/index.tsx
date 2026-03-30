import React from 'react';
import {
  Footer,
  Header,
  HeaderBg,
} from '@/components/HomePage';
import { MainLayoutContainer } from './index.styled';

export interface MainLayoutProps extends React.PropsWithChildren {
  activeMenuId?: string;
}

export function MainLayout({
  children,
  activeMenuId,
}: MainLayoutProps): JSX.Element {

  return (
    <MainLayoutContainer className="main-layout">
      <Header activeMenuId={activeMenuId} />
      <HeaderBg />

      <section className='bg-white'>
        {children}
      </section>

      <Footer />
    </MainLayoutContainer>
  );
}


