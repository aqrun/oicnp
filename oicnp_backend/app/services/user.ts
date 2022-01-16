import { gql } from '@apollo/client';
import { client } from '~/utils';
import {
  FetchUserLoginReponseData,
  FetchUserLoginRequestParams,
} from '~/typings';

export const USER_LOGIN_QUERY = gql`
  query userLogin(
    $username: String,
    $email: String,
    $password: String!,
  ) {
    userLogin(
      username: $username,
      email: $email,
      password: $password,
    ) {
      uid,
      username,
      email,
      status,
      admin,
      intro,
      lastLoginOn,
      mustChangePassword,
      passwordChangedOn,
      createdAt,
      updatedAt,
    }
  }
`;

export const fetchUserLogin = (
  options: Partial<FetchUserLoginRequestParams> = {}
) => {
  const variables = {
    ...options,
  };

  return new Promise<FetchUserLoginReponseData>((resolve, reject) => {
    client.query({
      query: USER_LOGIN_QUERY,
      variables,
    }).then(res => {
      resolve({
        user: res?.data?.userLogin as any,
      });
    }).catch(err => {
      resolve({
        user: undefined,
        err,
      });
    })
  });
}