import { gql } from '@apollo/client';
import {
  client,
  generateListResponseData,
} from '../utils';
import { get } from 'lodash';
import {
  QueryNodesRequestParams,
  QueryNodesResponseData,
  QueryNodeRequestParams,
  QueryNodeResponseData,
} from '../typings';

const NODE = `
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
`;

export const NODES_QUERY = gql`
  query NodeList(
    $category: String,
    $page: Int,
    $pageSize: Int,
    $targetNid: Int,
    $orderName: String,
    $orderDir: String,
  ) {
    nodes(
      category: $category, page: $page, pageSize: $pageSize,
      orderName: $orderName, orderDir: $orderDir,
      targetNid: $targetNid
    ) {
      edges {
        node {
          ${NODE}
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

export const NODE_QUERY = gql`
  query Node(
    $bundle: String,
    $nid: Int,
    $vid: String,
  ) {
    node(bundle: $bundle, nid: $nid, vid: $vid) {
      ${NODE}
    }
  }
`;

export const queryNode = (options: Partial<QueryNodeRequestParams> = {}) => {
  const variables = {
    ...options,
  };
  return new Promise<QueryNodeResponseData>((resolve, reject) => {
    client.query({
      query: NODE_QUERY,
      variables,
    }).then(res => {
      resolve(res?.data);
    }).catch(err => {
      reject(err);
    })
  });
}