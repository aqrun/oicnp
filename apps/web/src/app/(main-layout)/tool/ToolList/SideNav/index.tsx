import { TOOL_CATEGORIES } from '@/content/tools';
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
      <ul>
        {TOOL_CATEGORIES?.map((item) => {
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
  item: typeof TOOL_CATEGORIES[0];
  active?: boolean;
}

function SideNavItem({
  item,
  active = false,
}: SideNavItemProps): JSX.Element {
  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a href={`/tool/${item?.id}`}>
        <span className="item-name">
          {item?.name}
        </span>
      </a>
    </li>
  );
}