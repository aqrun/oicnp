import React, { useEffect, useRef, useState, useMemo } from 'react';
import { mainMenu, IS_CLIENT, SITE } from '../../constants';
import Link from 'next/link';
import { useScroll, useMemoizedFn } from 'ahooks';

export interface HeaderBaseProps {
  menuId: number;
  activeVid?: string;
}

const HeaderBase: React.FC<HeaderBaseProps> = ({
  menuId,
  activeVid,
}) => {
  const headerRef = useRef<HTMLDivElement>(null);
  const domScroll = useScroll(document);
  const [scrollFlag, setScrollFlag] = useState(domScroll?.top ?? 0);
  const [headerDownCls, setHeaderDownCls] = useState('');
  const [headerUpCls, setHeaderUpCls] = useState('');
  const [logoStyle, setLogoStyle] = useState({});
  const [navCls, setNavCls] = useState('');
  const [navShow, setNavShow] = useState(false);

  const scrollHandle = useMemoizedFn(() => {
    const scrollTop = domScroll?.top ?? 0;
    const headerHeight = headerRef.current?.offsetHeight ?? 0;
    const themeStyle = 'default';
    
    if (scrollTop > headerHeight) {
      if(scrollTop > 3 * headerHeight) {
        setHeaderUpCls('headerUp');
        setLogoStyle({
          background: `url(/assets/icons/logo_${themeStyle}.svg) no-repeat center`,
        });
        setNavCls(`nav-${themeStyle}`);
      }
    } else {
      setHeaderUpCls('header-up-removed');
      setLogoStyle({
        background: 'url(/assets/icons/logo.svg) no-repeat center',
      });
      setNavCls('');
    }

    if (scrollFlag > scrollTop) {
      setHeaderDownCls('headerDown');
    } else {
      setHeaderDownCls('');
    }
    setScrollFlag(scrollTop);
  });

  const headerCls = useMemo(() => {
    return `g-header ${headerDownCls} ${headerUpCls}`;
  }, [headerDownCls, headerUpCls]);

  const toggleClickHandle = useMemoizedFn((e) => {
    e?.stopPropagation();
    setNavShow(!navShow);
  });

  const domClickHandle = useMemoizedFn(() => {
    setNavShow(false);
  });

  useEffect(() => {
    if (IS_CLIENT && window.innerWidth > 695) {
      scrollHandle();
    }
    window.document.addEventListener('click', domClickHandle);

    return () => {
      window.document.removeEventListener('click', domClickHandle);
    };
  }, [domScroll?.top]);

  return (
    <header className={headerCls} ref={headerRef}>
      <div className="g-logo" style={logoStyle}>
        <Link href="/">{''}</Link>
      </div>
      <i
        id="menu-toggle"
        className="iconfont g-icon-menu icon-menu-right"
        onClick={toggleClickHandle}
      ></i>
      <nav className={`g-nav ${navCls} ${navShow ? 'oic-mobile-show': ''}`}>
        <ul>
          {mainMenu.map((item) => {
            let cls = menuId === item.id ? ' active': '';
            
            if (activeVid) {
              cls = activeVid === item.vid ? ' active' : '';
            }
            return (
              <li
                key={item.id}
                className={`id_${item.id}${cls} vid-${item.vid}`}
              >
                <Link href={item.href}>{item.name}</Link>
              </li>
            );
          })}
        </ul>
      </nav>
    </header>
  );
};

export default HeaderBase;
