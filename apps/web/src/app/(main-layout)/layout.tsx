import React from 'react';
import {
  Footer,
  Header,
  HeaderBg,
} from '@/components/HomePage';
import { MainLayoutContainer } from './index.styled';

/**
 * 可能需要权限检测
 */
export default async function MainLayout({ children }: React.PropsWithChildren): Promise<JSX.Element> {

  return (
    <MainLayoutContainer className="main-layout">
      <Header />
      <HeaderBg />

      <section className='bg-white'>
        {children}
      </section>

      <Footer />
    </MainLayoutContainer>
  );
}


