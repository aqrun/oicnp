import { create } from 'zustand';
import { BaseState } from './types';
import { MenuItem } from '~/types';
import { menus } from './menus';

export interface BaseAppState {
  /**
   * 所有菜单数据
   * 
   * 最多三级
   * 
   * * 第一级是头部导航显示
   * * 第二级是侧面导航显示
   * * 第三级是侧面展开内容
   */
  menus: MenuItem[];
  /**
   * 选中的主菜单
   */
  mainMenuKey?: string;
  /**
   * 选中的侧导航项
   */
  sideMenuKey?: string;
  /**
   * 侧导航展开项
   */
  sideMenuOpenKey?: string;
}

export type AppState = BaseAppState & BaseState<BaseAppState>;

/**
 * 应用主状态数据
 */
export const useAppStore = create<AppState>()((set) => ({
  menus: menus,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
