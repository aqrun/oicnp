import { useLocalStorageState, useMemoizedFn } from 'ahooks';
import { AuthState } from '~/types';
import { useAppStore } from '~/stores';

/**
 * 临时存储使用内存store存储
 * 记住时长久存储使用 localstorage
 */
export const useAuthState = () => {
  const storeAuthState = useAppStore((state) => state.authState);
  const setStoreAuthState = useAppStore((state) => state.setState);
  const [localStorageAuthState, setLocalStorageAuthState] = useLocalStorageState<AuthState | undefined>(
    'oic-admin-user-auth',
    undefined,
  );

  /**
   * 记住时使用 local storage 存储
   */
  const setAuthState = useMemoizedFn((authState: AuthState) => {
    if (authState?.remember) {
      setLocalStorageAuthState(authState);
    } else {
      setStoreAuthState({
        authState,
      });
    }
  });

  /**
   * 登出或过期时 重置状态
   */
  const resetAuthState = useMemoizedFn(() => {
    setLocalStorageAuthState(undefined);
    setStoreAuthState({
      authState: undefined,
    });
  });

  return {
    authState: localStorageAuthState || storeAuthState,
    setAuthState,
    resetAuthState,
  };
};
