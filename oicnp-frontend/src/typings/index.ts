export * from './nodes';

export interface Blog {
  slug: string;
  date: string;
  file: string;
  filePath: string;
  title: string;
  tags: string[];
  excerpt: string;
  category: {
    name: string;
    dir: string;
  };
  content?: string;
}

export enum MenuId {
  index = 100,
  backend = 101,
  frontend = 102,
  server = 103,
  rust = 104,
  diary = 105,
}