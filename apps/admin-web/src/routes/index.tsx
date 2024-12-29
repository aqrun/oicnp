import { useEffect } from 'react';
import { Routes, Route, useParams } from 'react-router';
import {
  MainLayout,
} from '~/components';
import {
  r,
} from '~/utils';
import useMenus from './useMenus';
import Login from '~/pages/Login';
import RouteElement from './RouteElement';
import { SubMenuType, MenuItem, RoutePathParams } from '~/types';

export default function AppRoutes(): JSX.Element {
  const pathParams = useParams() as RoutePathParams
  console.log('pa--', pathParams)
  const {
    mainMenuKey,
    sideMenuOpenKey,
    sideMenuKey,
  } = pathParams;

  const { menus, mainMenus, sideMenus } = useMenus(pathParams);

  // 默认菜单项
  const defaultMainMenu = menus?.[0];
  const defaultSideOpen = defaultMainMenu?.children?.[0];
  const defaultSideMenu = defaultSideOpen?.children?.[0];

  const mainLayout = (
    <MainLayout
      mainMenus={mainMenus}
      sideMenus={sideMenus}
      mainMenuKeys={[mainMenuKey || `${defaultMainMenu?.key}`]}
      sideMenuOpenKeys={[sideMenuOpenKey || `${defaultSideOpen?.key}`]}
      sideMenuKeys={[sideMenuKey || `${defaultSideMenu?.key}`]}
    />
  );

  useEffect(() => {
    console.log('pa', pathParams)
  }, [pathParams]);

  return (
    <Routes>
      <Route path={r('/')} element={mainLayout} >
        <Route path="login" element={<Login />} />
        <Route
          path={`:mainMenuKey?/:sideMenuOpenKey?/:sideMenuKey?`}
          element={(
            <RouteElement
              mainMenuKey={mainMenuKey || `${defaultMainMenu?.key}`}
              sideMenuOpenKey={sideMenuOpenKey || `${defaultSideOpen?.key}`}
              sideMenuKey={sideMenuKey || `${defaultSideMenu?.key}`}
            />
          )}
        />
      </Route>
    </Routes>
  );

  return (
    <Routes>
      <Route path={r('/')} element={mainLayout} >
        <Route path="login" element={<Login />} />
        {/* 主菜单路由 */}
        {menus?.map((item) => {
          return (
            <Route
              key={`${item.key}`}
              path={`${item.key}`}
              element={item?.label}
            >
              {/* 侧导航路由 */}
              {(item as SubMenuType)?.children?.map((n) => {
                const nItem = n as unknown as MenuItem;

                // 存在二级菜单项
                // 侧导航最多支持2级
                if ((nItem as SubMenuType)?.children?.length) {
                  return (
                    <Route 
                      key={`${nItem.key}`}
                      path={`${nItem.key}`}
                      element={nItem?.label}
                    >
                      
                      {(nItem as SubMenuType)?.children?.map((j) => {
                        const jItem = j as unknown as MenuItem;
                        return (
                          <Route 
                            key={`${jItem.key}`}
                            path={`${jItem.key}`}
                            element={jItem?.label}
                          />
                        );
                      })}
                    </Route>
                  );
                } else {
                  // 侧导航单页菜单项 无二级菜单项 不可展开
                  return (
                    <Route 
                      key={`${nItem.key}`}
                      path={`${nItem.key}`}
                      element={nItem?.label}
                    />
                  );
                }
              })}
            </Route>
          );
        })}
      </Route>
    </Routes>
  );
}
