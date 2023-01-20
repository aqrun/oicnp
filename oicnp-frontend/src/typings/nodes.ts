import {
  User,
} from './users';
import { Taxonomy } from './taxonomies';

export interface Node {
  nid: number;
  vid: string;
  bundle: string;
  title: string;
  subTitle?: string;
  cover?: string;
  viewed: number;
  deleted: boolean;
  author: User;
  createdBy: User;
  updatedBy: User;
  createdAt: string;
  updatedAt: string;
  category: Taxonomy;
  nodeBody: NodeBody;
  tags: Taxonomy[];
}

export interface NodeBody {
  nid: number;
  summary: string;
  body: string;
  bodyFormat: string;
}

export interface QueryNodesRequestParams {
  category?: string;
  page?: number;
  pageSize?: number;
  targetNid?: number;
  orderName?: string;
  orderDir?: string;
}

export interface QueryNodesResponseData {
  nodes: Node[];
  page: number;
  pageSize: number;
  totalCount: number;
}

export interface QueryNodeRequestParams {
  bundle: string;
  nid: number;
  vid: string;
}

export interface QueryNodeResponseData {
  node: Node;
}