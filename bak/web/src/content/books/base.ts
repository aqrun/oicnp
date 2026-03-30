import {
  BookCategories,
} from './types';

export const BOOK_CATEGORIES: BookCategories[] = [
  {
    id: 'all',
    name: '全部',
  },
  {
    id: 'wu-dai-shi-ci',
    name: '五代诗词',
    tags: ['花间集', '南唐'],
    dynasty: '五代',
  },
  {
    id: 'tang-shi',
    name: '唐诗',
    tags: ['全唐诗,全宋诗'],
    dynasty: '唐',
  },
  {
    id: 'song-ci',
    name: '宋词',
    tags: ['宋词'],
    dynasty: '宋',
  },
  {
    id: 'chu-ci',
    name: '楚辞',
    tags: ['楚辞'],
    dynasty: '先秦',
  },
  {
    id: 'meng-xue',
    name: '蒙学',
    tags: ['蒙学'],
  },
  {
    id: 'si-shu-wu-jing',
    name: '四书五经',
    tags: ['四书五经', '诗经', '论语'],
  },
];