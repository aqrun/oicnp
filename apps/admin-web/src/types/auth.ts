/**
 * 本地存储用户状态
 */
export interface AuthState {
  username: string;
  token: string;
  uuid: string;
  expireTime: number;
}