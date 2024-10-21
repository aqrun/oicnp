import { useEffect } from 'react';
import { useLocalStorageState, useMemoizedFn } from 'ahooks';
import { useRecoilState } from 'recoil';
import type { AuthState } from '~/types';
import { authState as authStateAtom} from '~/atoms';

/**
 * 登录状态存储
 * @returns [authState, setAuthState]
 */
export const useAuthState = (needInit = false) => {
  const [recoilAuthState, setRecoilAuthState] = useRecoilState(authStateAtom);
  const [authState, setAuthState] = useLocalStorageState<AuthState>(
    'oicnp-auth-state',
    {
      defaultValue: {},
    },
  );

  /**
   * 同时更新localStorage和recoil state
   */
  const updateAuthState = useMemoizedFn((paramAuth: AuthState) => {
    setAuthState(paramAuth);
    setRecoilAuthState(paramAuth);
  });

  /**
   * 同步登录状态
   */
  const initStateData = useMemoizedFn(() => {
    setRecoilAuthState({
      ...authState,
      initialized: true,
    });
  });

  useEffect(() => {
    // local storage 有数据时 初始化一次数据到recoil state
    if (authState?.user && needInit && !recoilAuthState.initialized) {
      initStateData();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps -- desc
  }, [authState]);

  return [recoilAuthState, updateAuthState] as const;
};
