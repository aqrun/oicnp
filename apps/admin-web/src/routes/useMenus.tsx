import {
  MenuItem,
  SubMenuType,
} from '~/types';
import { useGlobalState } from '~/context';
import {
  Icon,
} from '~/components';

/**
 * 菜单相关数据
 */
export default function useMenus() {
  const { urlState, menus } = useGlobalState();

  // 主菜单数据是一次菜单数据
  const mainMenus = (menus || [])?.map((item) => {
    // 主菜单移除子项
    const newItem: MenuItem = {
      ...item,
      icon: item?.icon ? (<Icon icon={item?.icon as string} />) : undefined,
      children: undefined,
    };


    delete newItem['ignore'];

    return newItem;
  });
  const activeMainMenu = urlState?.mainMenu;
  // console.log('activeMainMenu', activeMainMenu, 'urlState', urlState)
  // 侧边导航菜单是二级菜单数据
  const sideMenus = (((activeMainMenu as SubMenuType)?.children || []) as MenuItem[])?.map((item) => {
    let subItems: MenuItem[] = [];

    if (item?.children) {
      subItems = item?.children?.map((n) => {
        const subNewItem: MenuItem = {
          ...n,
          key: `${item?.key}@${n?.key}`,
        };

        if (n?.icon) {
          subNewItem.icon = (<Icon icon={item?.icon as string} />);
        }

        return subNewItem;
      })
    }

    // 侧导航第一级
    const newItem: MenuItem = {
      ...item,
      icon: (<Icon icon={item?.icon as string} />),
    };

    if (subItems?.length) {
      newItem.children = subItems;
    }

    return newItem;
  });

  return {
    menus,
    mainMenus,
    sideMenus,
  };
};
