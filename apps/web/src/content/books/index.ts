import { SHU_CI_ITEMS } from './shu_ci';
import { LUN_YU_ITEMS } from './lun_yu';
import { MENG_XUE_ITEMS } from './meng_xue';
import { SHI_JING_ITEMS } from './shi_jing';
import { SHUI_MO_TANG_SHI_ITEMS } from './shui_mo_tang_shi';
import { SI_SHU_WU_JING_ITEMS } from './si_shu_wu_jing';

export * from './base';
export * from './types';

export const ALL_BOOKS = [
  ...SHU_CI_ITEMS,
  ...LUN_YU_ITEMS,
  ...MENG_XUE_ITEMS,
  ...SHI_JING_ITEMS,
  ...SHUI_MO_TANG_SHI_ITEMS,
  ...SI_SHU_WU_JING_ITEMS,
];