'use client';

import {
  QueryClientProvider,
  QueryClient,
} from '@tanstack/react-query';
import { GlobalContext, GlobalState } from '@/context';
import { parseHashState } from '@/utils/app.client';

export interface AppProviderProps extends React.PropsWithChildren {

}

const queryClient = new QueryClient();

/**
 * 全局provider注册
 */
export function AppProvider({
  children,
}: AppProviderProps) {
  // 全局状态参数
  const globalState: GlobalState = {
    hashState: parseHashState(),
  };

  return (
    <GlobalContext.Provider
      value={globalState}
    >
      <QueryClientProvider
        client={queryClient}
      >
        {children}
      </QueryClientProvider>
    </GlobalContext.Provider>
  );
}
