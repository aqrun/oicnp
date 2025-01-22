
export interface User {
  username: string;
}

export interface AuthState {
  user?: User;
  initialized?: boolean;
}
