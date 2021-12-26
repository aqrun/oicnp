import { gql } from '@apollo/client';
import {
  client,
  generateListResponseData,
} from '../utils';
import { get } from 'lodash';
import {
  QueryNodesRequestParams,
  QueryNodesResponseData,
} from '../typings';

export const NODES_QUERY = gql`
  query NodeList(
    $category: String,
    $page: Int,
    $pageSize: Int
  ) {
    nodes(category: $category, page: $page, pageSize: $pageSize) {
      edges {
        node {
          nid,
          vid,
          bundle,
          title,
          viewed,
          deleted,
          createdBy {
            uid,
            username,
            nickname,
            email,
            avatar {
              fid,
              filename,
              uri,
            },
          },
          createdAt,
          updatedBy {
            uid,
            username,
            nickname,
            email,
            avatar {
              fid,
              filename,
              uri,
            },
          },
          updatedAt,
          author {
            uid,
            username,
            nickname,
            email,
            avatar {
              fid,
              filename,
              uri,
            },
          },
          category {
            tid,
            vid,
            name,
            bundle,
          },
          nodeBody {
            nid,
            summary,
            body,
            bodyFormat,
          },
          tags {
            tid,
            vid,
            name,
            count,
          },
        }
      },
      page,
      pageSize,
      totalCount,
    }
  }
`;

export const queryNodes = (options: Partial<QueryNodesRequestParams> = {}) => {
  const variables = {
    ...options,
  };
  return new Promise<QueryNodesResponseData>((resolve, reject) => {
    client.query({
      query: NODES_QUERY,
      variables,
    }).then(res => {
      resolve(generateListResponseData<QueryNodesResponseData>(res, 'nodes'));
    }).catch(err => {
      reject(err);
    })
  });
}