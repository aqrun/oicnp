import { RoutePathParams } from '~/types';
import {
  useAppStore,
} from '~/stores';

export type RouteElementProps = {
  name?: string,
} & RoutePathParams;

export default function RouteElement({
  mainMenuKey,
  sideMenuOpenKey,
  sideMenuKey,
}: RouteElementProps): JSX.Element {
  const { menus } = useAppStore();

  // 主菜单项
  const mainMenu = menus?.find((item) => {
    return item?.key === mainMenuKey;
  });
  // 侧栏一级
  const sideOpenMenu = (mainMenu?.children || [])?.find((item) => {
    return item?.key === sideMenuOpenKey;
  });
  // 侧栏二级
  const sideMenu = (sideOpenMenu?.children || [])?.find((item) => {
    return item?.key === sideMenuKey;
  });

  // 只有主菜单单页显示
  if (mainMenuKey && !sideMenuOpenKey && !sideMenuKey) {
    return (
      <>
        {mainMenu?.label}
      </>
    );
  } else if (mainMenuKey && sideMenuOpenKey && !sideMenuKey) {
    return (
      <>
        {sideOpenMenu?.label}
      </>
    );
  } else if (mainMenuKey && sideMenuOpenKey && sideMenuKey) {
    return (
      <>
        {sideMenu?.label}
      </>
    );
  }

  return (
    <>
      404 not found
    </>
  );
}
