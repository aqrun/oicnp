import { MenuItem } from '~/types';

const main: MenuItem[] = [
  {
    key: 'dashboard',
    icon: 'LaptopOutlined',
    label: '仪表盘',
  },
  {
    key: 'users',
    icon: 'UserOutlined',
    label: '用户',
    children: [
      {
        key: 'list',
        icon: '',
        label: '列表',
      },
    ],
  },
  {
    key: 'roles',
    icon: 'UserOutlined',
    label: '角色',
    children: [
      {
        key: 'list',
        icon: '',
        label: '列表',
      },
    ],
  },
];

const cms: MenuItem[] = [
  {
    key: 'posts',
    label: '文章',
    icon: '',
    children: [
      {
        key: 'list',
        icon: '',
        label: '列表',
      },
    ],
  },
  {
    key: 'categories',
    label: '分类',
    icon: '',
  },
  {
    key: 'tags',
    label: '标签',
    icon: '',
  },
];

export const menus: MenuItem[] = [
  {
    key: 'main',
    label: '控制台',
    children: main,
  },
  {
    key: 'cms',
    label: '内容管理',
    children: cms,
  },
  {
    key: 'settings',
    label: '设置',
  },
];