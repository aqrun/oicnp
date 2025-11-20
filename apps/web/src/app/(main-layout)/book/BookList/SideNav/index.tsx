'use client';

import { useRouter } from 'next/navigation';
import { BOOK_CATEGORIES } from '@/content/books';
import clsx from 'clsx';
import { useBookStore } from '../../useBookStore';
import {
  SideNavContainer
} from './index.styled';

export interface SideNavProps {
  catVid?: string;
}

export default function SideNav({
  catVid = 'all',
}: SideNavProps): JSX.Element {
  const category = useBookStore((state) => state.category);

  return (
    <SideNavContainer>
      <ul>
        {BOOK_CATEGORIES?.map((item) => {
          return (
            <SideNavItem
              key={item?.id}
              item={item}
              active={category === item?.id}
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
  const setBookState = useBookStore.setState;

  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a
        onClick={() => {
          setBookState({ category: item?.id, title: '' });

          if (item?.id === 'all') {
            router.push(`/book`);
          } else {
            router.push(`/book#/category/${item?.id}`);
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