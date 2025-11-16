'use client';

import { useEffect } from 'react';
import { usePathname } from 'next/navigation';

/**
 * 菜单高亮处理
 */
export function HeaderNavHandler() {
  const pathname = usePathname();
  console.log('pathname--', pathname);

  useEffect(() => {
    if (window.innerWidth < 1280) return;
    const $s = document.querySelectorAll('.header-nav-item');
    $s?.forEach((item) => {
      item?.classList.remove('active');
    });

    let vid = 'home';

    if (pathname === '/tool') {
      vid = 'tool';
    } else if (pathname === '/book') {
      vid = 'book';
    } else if (pathname === '/rust' || pathname === '/cat/rust') {
      vid = 'rust';
    } else if (pathname === '/blog'
      || pathname?.indexOf('/cat/') === 0
      || pathname?.indexOf('/n/') === 0
      || pathname?.indexOf('/p/') === 0
    ) {
      vid = 'blog';
    }

    const $item = document.querySelector(`.header-nav-item.item-${vid}`);
    if ($item) {
      $item?.classList.add('active');
    }
  }, [pathname]);

  return <></>;
}