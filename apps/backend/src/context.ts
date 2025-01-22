import { createContext, useContext } from 'react';
// import { UrlState } from '@/utils';
import { MenuItem, AuthState } from '@/types';

export interface GlobalState {
  menus?: MenuItem[];
  // urlState?: UrlState;
  authState?: AuthState;
  setAuthState?: (authState: AuthState) => void;
}

const initialGlobalData: GlobalState = {
  
};

export const GlobalContext = createContext<GlobalState>(initialGlobalData);

export function useGlobalState(): GlobalState {
  const context = useContext(GlobalContext);
  return context;
}
