import { atom } from 'recoil';
import type { AuthState } from '~/types';

export const authState = atom<AuthState>({
  key: 'userAuth',
  default: {
    user: undefined,
    initialized: false,
  },
});