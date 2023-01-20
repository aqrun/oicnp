import { MenuId } from '../typings';

export const mainMenu = [
  { id: MenuId.index, name: '首页', href: '/', vid: 'index' },
  { id: MenuId.backend, name: '后端开发', href: '/blogs/backend', vid: 'backend' },
  { id: MenuId.frontend, name: '前端开发', href: '/blogs/frontend', vid: 'frontend' },
  { id: MenuId.server, name: '服务器', href: '/blogs/server', vid: 'server' },
  { id: MenuId.rust, name: 'Rust', href: '/blogs/rust', vid: 'rust' },
  { id: MenuId.diary, name: '随笔', href: '/blogs/diary', vid: 'diary' },
];
