'use client';

import { createContext, useContext } from 'react';
import { HashState } from '@/types';
import { HookAPI } from 'antd/lib/modal/useModal';
import { MessageInstance } from 'antd/lib/message/interface';

export interface GlobalState {
  /**
   * 解析后的哈析参数
   */
  hashState?: HashState;
  modal: HookAPI;
  message: MessageInstance;
}

const initialGlobalData: GlobalState = {
  modal: null as unknown as HookAPI,
  message: null as unknown as MessageInstance,
};

export const GlobalContext = createContext<GlobalState>(initialGlobalData);

export function useGlobalState(): GlobalState {
  const context = useContext(GlobalContext);
  return context;
}
