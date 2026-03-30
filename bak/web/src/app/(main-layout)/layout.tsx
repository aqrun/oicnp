import React from 'react';

/**
 * 可能需要权限检测
 */
export default async function MainLayout({ children }: React.PropsWithChildren): Promise<JSX.Element> {

  return (
    <>
      {children}
    </>
  );
}


