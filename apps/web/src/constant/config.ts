export const siteConfig = {
  title: '灵犀纪',
  description:
    '灵犀纪,rust语言开发,服务器,html,css,js,nextjs,tailwindcss',
  /** Without additional '/' on the end*/
  url: 'https://www.lxage.com',
};

/** 首页文章列表每页显示条数 */
export const HOME_PAGE_SIZE = 10;
/** 分类页面分页 */
export const CATEGORY_PAGE_SIZE = 5;

export const MAIN_MENUS = [
  {
    name: '首页',
    vid: 'home',
    href: '/',
  },
  {
    name: '常用工具',
    vid: 'tool',
    href: '/tool/',
  },
  {
    name: '内容阅读',
    vid: 'book',
    href: '/book/',
  },
  {
    name: 'IT技术',
    vid: 'blog',
    href: '/blog/',
  },
  {
    name: 'Rust语言',
    vid: 'rust',
    href: '/cat/rust/',
  },
];

export const CATEGORIES = [
  {
    name: '综合',
    vid: 'all',
    href: '/blog/',
  },
  {
    name: 'Rust语言',
    vid: 'rust',
    href: '/cat/rust/',
  },
  {
    name: '服务器',
    vid: 'server',
    href: '/cat/server/',
  },
  {
    name: '后端开发',
    vid: 'backend',
    href: '/cat/backend/',
  },
  {
    name: '前端开发',
    vid: 'frontend',
    href: '/cat/frontend/',
  },
  {
    name: '每日随笔',
    vid: 'diary',
    href: '/cat/diary/',
  },
];
