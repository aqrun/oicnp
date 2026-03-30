import fs from 'fs';
import matter from 'gray-matter';
import path from 'path';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import { unified } from 'unified';

import { MAIN_MENUS } from '@/constant';

const CATEGORIES = ['diary', 'rust', 'backend', 'frontend', 'server'];
const REG_DATE = /\d{4}-\d{2}-\d{2}/i;

export const content_path = path.join(process.cwd(), 'src/content');
export const pages_path = path.join(content_path, 'pages');
export const reading_path = path.join(content_path, 'reading');

export interface Node {
  content?: string;
  isEmpty?: boolean;
  excerpt?: string;
  file_path?: string;
  data?: {
    title?: string;
    description?: string;
    date?: Date;
    slug?: string;
    /** 图片 */
    thumb?: string;
    weight?: number;
    startedAt?: string;
    book?: string;
    taxonomies?: {
      categories?: string[];
      tags?: string[];
    };
  };
}

/**
 * 获取读书列表数据
 */
export const getBookList = () => {
  const bookInfos = fs.readdirSync(reading_path)?.map((bookItem) => {
    const book_path = path.join(reading_path, bookItem);
    const infoFile = path.join(book_path, '_index.md');

    const info_content = fs.readFileSync(infoFile, 'utf-8');
    const matterResults = matter(info_content);

    const nodeItem: Node = {
      file_path: book_path,
      data: {
        ...matterResults.data,
        book: bookItem,
      },
    };

    return nodeItem;
  });
  return bookInfos;
};

/**
 * 获取书籍所有页
 */
export const getBookPages = (bookName: string) => {
  const pages: Node[] = [];

  const bookDir = path.join(reading_path, bookName)
  fs.readdirSync(bookDir)?.filter((item) => {
    return !item?.startsWith('_index.md');
  })?.forEach((fileName) => {
    const file_path = path.join(bookDir, fileName);

    const info_content = fs.readFileSync(file_path, 'utf-8');
    const matterResults = matter(info_content);
    const file_name_meta = parseFileName(fileName);

    pages.push({
      ...matterResults,
      file_path,
      data: {
        ...(matterResults?.data || {}),
        date: matterResults?.data?.date || file_name_meta?.date,
        slug: matterResults?.data?.slug || file_name_meta?.slug,
        book: bookName,
      },
    });
  });
  return pages;
};

/**
 * 递归遍历目录
 */
const getAllNodeFilesRecursive = (
  dir: string, // 目录路径
  callback: (file: string, dir_name: string, file_name: string) => void
) => {
  const state = fs.statSync(dir);

  if (state.isFile()) {
    const full_dir_name = path.dirname(dir);
    const contentIndex = full_dir_name.indexOf('/content/');
    const dir_arr = full_dir_name.slice(contentIndex)?.split('/');
    dir_arr.shift(); // ''
    dir_arr.shift(); // content

    const dir_name = dir_arr.shift() || '';
    const file_name = path.normalize(dir)?.split('/')?.pop() || '';
    callback(dir, dir_name, file_name);
  } else if (state.isDirectory()) {
    fs.readdirSync(dir)?.forEach((subItem) => {
      const newDir = path.join(dir, subItem);
      getAllNodeFilesRecursive(newDir, callback);
    });
  }
}

export const getAllNodes = () => {
  const all_matter_results: Node[] = [];

  getAllNodeFilesRecursive(content_path, (file_path: string, dir_name: string, file_name: string) => {
    if (!CATEGORIES?.includes(dir_name)) {
      return;
    }
    if (file_name?.startsWith('_index.md')) {
      return;
    }

    const file_contents = fs.readFileSync(file_path, 'utf-8');

    const matterResults = matter(file_contents);
    const file_name_meta = parseFileName(file_name);
  
    all_matter_results.push({
      ...matterResults,
      file_path,
      data: {
        ...(matterResults?.data || {}),
        date: matterResults?.data?.date || file_name_meta?.date,
        slug: matterResults?.data?.slug || file_name_meta?.slug,
      },
    });
  });

  return all_matter_results;
};

export interface ListDataOptions {
  category?: string;
  tag?: string;
  page?: number;
  pageSize?: number;
  orderBy?: string;
  order?: 'asc' | 'desc';
}

export const getNodes = (options: ListDataOptions = {}) => {
  const allNodes = getAllNodes();
  let validNodes = allNodes;

  validNodes = validNodes?.filter((item) => {
    if (options?.category) {
      return item?.data?.taxonomies?.categories?.includes(options?.category);
    }

    if (options?.tag) {
      return item?.data?.taxonomies?.tags?.includes(options?.tag);
    }

    return true;
  });

  if (options?.orderBy === 'date') {
    validNodes = validNodes.sort((a, b) => {
      const aDate = a?.data?.date?.valueOf() || 0;
      const bDate = b?.data?.date?.valueOf() || 0;

      if (options?.order === 'asc') {
        return aDate - bDate;
      } else {
        return bDate - aDate;
      }
    });
  }

  const page = options?.page || 1;
  const pageSize = options?.pageSize || 0;

  if (pageSize) {
    const startIndex = (page - 1) * pageSize;
    const endIndex = startIndex + pageSize;
    validNodes = validNodes?.slice(startIndex, endIndex);
  }

  return validNodes;
};

export interface FileNameMeta {
  date?: Date;
  slug?: string;
}

/**
 * 获取文件名中的日期
 */
export const parseFileName = (param_file_name: string): FileNameMeta => {
  const file_name_arr = param_file_name?.split('/');
  const file_name = file_name_arr?.pop() || param_file_name;
  let date;
  let slug = file_name;
  const matches = file_name?.match(REG_DATE);

  if (matches?.[0]) {
    date = new Date(matches?.[0]);
  }

  slug = slug
    .replace(/.mdx?/i, '') // 去除后缀
    .replace(REG_DATE, '') // 去除日期
    .replace(/[.\s]/i, '-') // 去除特殊字符
    .replace(/^-/, ''); // 去除前缀

  return {
    date,
    slug,
  };
};

/**
 * 分类和标签数据统计
 */
export const getTaxonomiesCount = () => {
  const allNodes = getAllNodes();
  const categories_counts: { [key: string]: number } = {};
  const tags_counts: { [key: string]: number } = {};

  allNodes?.forEach((node) => {
    node?.data?.taxonomies?.categories?.forEach((item) => {
      const count = categories_counts?.[item] || 0;
      categories_counts[item] = count + 1;
    });
    node?.data?.taxonomies?.tags?.forEach((item) => {
      const count = tags_counts?.[item] || 0;
      tags_counts[item] = count + 1;
    });
  });

  const categories = CATEGORIES?.map((vid) => {
    const menuItem = MAIN_MENUS?.find((item) => item?.vid === vid);

    return {
      ...menuItem,
      count: categories_counts?.[vid] || 0,
    };
  })?.sort((a, b) => {
    const a_count = a?.count || 0;
    const b_count = b?.count || 0;
    return b_count - a_count;
  });
  const tags = Object.keys(tags_counts)
    ?.map((tag) => {
      return {
        name: tag,
        count: tags_counts?.[tag] || 0,
        href: `/tags/${tag}`,
      };
    })
    ?.sort((a, b) => {
      const a_count = a?.count || 0;
      const b_count = b?.count || 0;
      return b_count - a_count;
    });

  return {
    categories,
    tags,
  };
};

/**
 * 获取分类具体信息
 */
export const getCategory = (vid: string | string[] | undefined) => {
  let validVid = vid;

  if (Array.isArray(vid)) {
    const vidArr = vid?.filter((item) => {
      return item !== 'article';
    });

    if (vidArr?.length) {
      validVid = vidArr?.[0];
    }
  }

  const target = MAIN_MENUS?.find((item) => {
    return item?.vid === validVid;
  });

  return target;
};

export const imageKeyWords = [
  'tree',
  'dog',
  'cat',
  'grass',
  'run',
  'jump',
  'water',
];

/**
 * 首页最新列表 每个分类一篇
 */
export const getNewsList = () => {
  const big_news_vid = 'rust';
  const rust_nodes = getNodes({
    category: big_news_vid,
    pageSize: 1,
    orderBy: 'date',
  });

  const extra_nodes = CATEGORIES?.filter((vid) => {
    return vid !== big_news_vid;
  })?.map((vid) => {
    const nodes = getNodes({
      category: vid,
      pageSize: 1,
      orderBy: 'date',
    });

    return nodes?.[0];
  });

  return [...rust_nodes, ...extra_nodes];
};

export const parseMarkdown = async (content: string) => {
  const data = await unified()
    .use(remarkParse) // Convert into markdown AST
    .use(remarkRehype) // Transform to HTML AST
    .use(rehypeSanitize) // Sanitize HTML input
    .use(rehypeStringify) // Convert AST into serialized HTML
    .process(content);

  return String(data);
};

