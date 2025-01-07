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
import { useMemoizedFn } from 'ahooks';

export default function AppRoot(): JSX.Element {
  const navigation = useNavigation();
  const [fetchInitData, iniLoading] = useAppInit();
  const { urlState, menus } = useUrlState();
  const {
    authState,
    setAuthState,
    resetAuthState,
  } = useAuthState();
  const apiLoading = useAppStore((state) => state.loading);

  const checkAuth = useMemoizedFn(() => {
    // 使用localstorage存储且到了过期时间重置
    if (authState?.remember
      && authState?.expireTime < Date.now()
    ) {
      resetAuthState();
    }
  });

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
  // 检测登录信息状态
  useEffect(() => {
    checkAuth();
  }, [authState, checkAuth]);
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
