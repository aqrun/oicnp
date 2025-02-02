import { createContext, useContext } from 'react';
import { HashState } from '@/types';

export interface GlobalState {
  /**
   * 解析后的哈析参数
   */
  hashState?: HashState;
}

const initialGlobalData: GlobalState = {
  
};

export const GlobalContext = createContext<GlobalState>(initialGlobalData);

export function useGlobalState(): GlobalState {
  const context = useContext(GlobalContext);
  return context;
}
