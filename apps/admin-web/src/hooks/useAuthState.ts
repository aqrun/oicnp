import { useLocalStorageState } from 'ahooks';
import { AuthState } from '~/types';

export const useAuthState = () => {
  const [authState, setAuthState] = useLocalStorageState<AuthState | undefined>(
    'oic-admin-user-auth',
    undefined,
  );

  let resAuthData = authState;

  // 是0 不记住
  if (!authState?.expireTime) {
    resAuthData = undefined;    
  } else if (authState?.expireTime < Date.now()) {
    // 超过过期时间
    resAuthData = undefined;
  }

  return {
    authState: resAuthData,
    setAuthState,
  };
};
