
export interface FetchUserLoginRequestParams {
  username?: string;
  email?: string;
  password: string;
};

export interface FetchUserLoginReponseData {
  user?: {
    uid: number;
    username: string;
    nickname: string;
    email: string;
    status: number;
    intro: string;
  };
  err?: any;
}
