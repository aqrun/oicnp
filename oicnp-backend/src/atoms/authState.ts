import { atom } from 'recoil';

export interface User {
  username: string;
}

export interface AuthState {
  user?: User;
}

export const authState = atom<AuthState>({
  key: 'userAuth',
  default: {
    user: undefined,
  },
});