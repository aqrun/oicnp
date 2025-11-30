import { ToolCategories } from '@/content/tools';
import clsx from 'clsx';
import {
  SideNavContainer
} from './index.styled';

export interface SideNavProps {
  catVid?: string;
  toolCategories?: ToolCategories[];
}

export default function SideNav({
  catVid = 'all',
  toolCategories,
}: SideNavProps): JSX.Element {
  return (
    <SideNavContainer>
      <ul>
        {toolCategories?.map((item) => {
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
  item: ToolCategories;
  active?: boolean;
}

function SideNavItem({
  item,
  active = false,
}: SideNavItemProps): JSX.Element {
  return (
    <li className={clsx("side-nav-item", active && "active")}>
      <a href={`/tool/t/${item?.id}`}>
        <span className="item-name">
          {item?.name}
        </span>
      </a>
    </li>
  );
}