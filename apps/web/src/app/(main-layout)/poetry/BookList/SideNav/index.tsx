'use client';

import { useRouter } from 'next/navigation';
import { BOOK_CATEGORIES } from '@/content/books';
import clsx from 'clsx';
import { useBookStore, defaultState } from '../../useBookStore';
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
      <ul>
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
  const router = useRouter();
  const setState = useBookStore.setState;

  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a
        onClick={() => {
          setState({
            ...defaultState,
          });

          if (item?.id === 'all') {
            router.push(`/poetry`);
          } else {
            router.push(`/poetry/category/${item?.id}`);
          }
        }}
      >
        <span className="item-name">
          {item?.name}
        </span>
      </a>
    </li>
  );
}