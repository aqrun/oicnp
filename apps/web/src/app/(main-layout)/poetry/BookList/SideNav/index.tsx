import { BOOK_CATEGORIES } from '@/content/books';
import clsx from 'clsx';
import {
  SideNavContainer
} from './index.styled';

export interface SideNavProps {
  catVid?: string;
}

export default function SideNav({
  catVid = 'all',
}: SideNavProps): JSX.Element {
  return (
    <SideNavContainer>
      <ul className="flex flex-row lg:flex lg:flex-col">
        {BOOK_CATEGORIES?.map((item) => {
          return (
            <SideNavItem
              key={item?.id}
              item={item}
              active={catVid === item?.id}
            />
          );
        })}
      </ul>
    </SideNavContainer>
  );
}

interface SideNavItemProps {
  item: typeof BOOK_CATEGORIES[0];
  active?: boolean;
}

function SideNavItem({
  item,
  active = false,
}: SideNavItemProps): JSX.Element {
  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a
        href={item?.id === 'all' ? '/poetry' : `/poetry/category/${item?.id}`}
      >
        <span className="item-name">
          {item?.name}
        </span>
      </a>
    </li>
  );
}