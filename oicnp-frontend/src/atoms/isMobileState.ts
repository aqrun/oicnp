import { atom } from 'recoil';

export const isMobileState = atom({
  key: 'isMobile',
  default: {
    value: false,
  },
});