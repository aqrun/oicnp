'use client';

import {
  QueryClientProvider,
  QueryClient,
} from '@tanstack/react-query';

export interface AppProviderProps extends React.PropsWithChildren {

}

const queryClient = new QueryClient();

/**
 * 全局provider注册
 */
export function AppProvider({
  children,
}: AppProviderProps) {
  return (
    <QueryClientProvider
      client={queryClient}
    >
      {children}
    </QueryClientProvider>
  );
}
