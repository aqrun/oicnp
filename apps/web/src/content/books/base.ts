import {
  BookCategories,
} from './types';

export const BOOK_CATEGORIES: BookCategories[] = [
  {
    id: 'all',
    name: '全部',
  },
  {
    id: 'wu_dai_shi_ci',
    name: '五代诗词',
    tags: ['花间集', '南唐'],
  },
  {
    id: 'tang_shi',
    name: '唐诗',
    tags: ['全唐诗'],
  },
  {
    id: 'song_ci',
    name: '宋词',
    tags: ['全宋诗'],
  },
  {
    id: 'chu_ci',
    name: '楚辞',
    tags: ['楚辞'],
  },
  {
    id: 'meng_xue',
    name: '蒙学',
    tags: ['蒙学'],
  },
  {
    id: 'si_shu_wu_jing',
    name: '四书五经',
    tags: ['四书五经', '诗经', '论语'],
  },
];