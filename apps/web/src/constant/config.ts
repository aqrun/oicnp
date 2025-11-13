export const siteConfig = {
  title: '光鹿跑冰',
  description:
    '子十个人博客,光鹿跑冰,rust语言开发,服务器,html,css,js,nextjs,tailwindcss',
  /** Without additional '/' on the end*/
  url: 'https://aqrun.com',
};

/** 首页文章列表每页显示条数 */
export const HOME_PAGE_SIZE = 5;
/** 分类页面分页 */
export const CATEGORY_PAGE_SIZE = 5;

export const MAIN_MENUS = [
  {
    name: 'Rust语言',
    vid: 'rust',
    href: '/category/rust/',
  },
  {
    name: '服务器',
    vid: 'server',
    href: '/category/server/',
  },
  {
    name: '后端开发',
    vid: 'backend',
    href: '/category/backend/',
  },
  {
    name: '前端开发',
    vid: 'frontend',
    href: '/category/frontend/',
  },
  {
    name: '每日随笔',
    vid: 'diary',
    href: '/category/diary/',
  },
  {
    name: '阅读小记',
    vid: 'reading',
    href: '/reading/',
  },
];
