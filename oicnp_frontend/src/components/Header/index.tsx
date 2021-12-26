import React from 'react';
import { mainMenu } from '../../constants';

export interface HeaderProps {
  menuId: number;
}

export const Header: React.FC<HeaderProps> = ({
  menuId,
}) => {

  return (
    <header className="g-header">
      <div className="g-logo">
        <a href="/"></a>
      </div>
      <i id="menu-toggle" className="iconfont icon-menu"></i>
      <nav className="g-nav">
        <ul>
          {mainMenu.map((item) => {
            const cls = menuId === item.id ? ' active': '';

            return (
              <li
                key={item.id}
                className={`id_${item.id}${cls}`}
              >
                <a href={item.href}>{item.name}</a>
              </li>
            );
          })}
        </ul>
      </nav>
    </header>
  );
}