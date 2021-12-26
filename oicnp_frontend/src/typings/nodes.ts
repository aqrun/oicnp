import {
  User,
} from './users';
import { Taxonomy } from './taxonomies';

export interface Node {
  nid: number;
  vid: string;
  bundle: string;
  title: string;
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
}

export interface QueryNodesResponseData {
  nodes: Node[];
  page: number;
  pageSize: number;
  totalCount: number;
}