import React, { useEffect } from 'react';
import { mainMenu } from '../../constants';
import Link from 'next/link';

export interface HeaderProps {
  menuId: number;
  activeVid?: string;
}

export const Header: React.FC<HeaderProps> = ({
  menuId,
  activeVid,
}) => {
  return (
    <header className="g-header">
      <div className="g-logo">
        <Link href="/">{''}</Link>
      </div>
      <i id="menu-toggle" className="iconfont icon-menu"></i>
      <nav className="g-nav">
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
}