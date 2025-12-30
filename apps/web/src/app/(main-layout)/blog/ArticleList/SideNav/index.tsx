import {
  CATEGORIES,
} from '@/constant';
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
      <ul className="flex flex-row lg:flex lg:flex-col flex-wrap">
        {CATEGORIES?.map((item) => {
          return (
            <SideNavItem
              key={item?.vid}
              item={item}
              active={catVid === item?.vid}
            />
          );
        })}
      </ul>
    </SideNavContainer>
  );
}

interface SideNavItemProps {
  item: typeof CATEGORIES[0];
  active?: boolean;
}

function SideNavItem({
  item,
  active = false,
}: SideNavItemProps): JSX.Element {
  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a href={`/cat/${item?.vid}`}>
        <span className="item-name">
          {item?.name}
        </span>
      </a>
    </li>
  );
}