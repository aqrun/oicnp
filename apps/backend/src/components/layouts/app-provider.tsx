'use client';

import {
  QueryClientProvider,
  QueryClient,
} from '@tanstack/react-query';
import { GlobalContext, GlobalState } from '@/context';
import { App } from 'antd';
import { ErrorHandler } from '../ErrorHandler';

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
    
  };

  return (
    <App>
      <GlobalContext.Provider
        value={globalState}
      >
        <QueryClientProvider
          client={queryClient}
        >
          {children}
          <ErrorHandler />
        </QueryClientProvider>
      </GlobalContext.Provider>
    </App>
  );
}
