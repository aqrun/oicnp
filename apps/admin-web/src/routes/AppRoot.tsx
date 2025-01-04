import { useEffect } from 'react';
import { Outlet, useNavigation } from 'react-router';
import {
  useAppInit,
  useUrlState,
  useAuthState,
} from '~/hooks';
import { GlobalContext, GlobalState } from '~/context';
import NProgress from 'nprogress';
import { useAppStore } from '~/stores';

export default function AppRoot(): JSX.Element {
  const navigation = useNavigation();
  const [fetchInitData, iniLoading] = useAppInit();
  const { urlState, menus } = useUrlState();
  const {
    authState,
    setAuthState,
  } = useAuthState();
  const apiLoading = useAppStore((state) => state.loading);

  // 初始化context数据
  const stateValue: GlobalState = {
    urlState,
    menus,
    authState,
    setAuthState,
  };

  // 监听路由跳转
  useEffect(() => {
    if (navigation.state === 'idle') {
      NProgress.done();
    } else {
      NProgress.start();
    }
  }, [navigation.state, navigation]);
  // 监听接口加载状态
  useEffect(() => {
    if (apiLoading) {
      NProgress.start();
    } else {
      NProgress.done();
    }
  }, [apiLoading]);
  // 全局数据初始化
  useEffect(() => {
    fetchInitData();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (iniLoading) {
    return (
      <>
        iniLoading...
      </>
    );
  }

  return (
    <GlobalContext.Provider value={stateValue}>
      <Outlet />
    </GlobalContext.Provider>
  );
};
