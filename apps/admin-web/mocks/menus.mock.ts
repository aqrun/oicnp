import { MenuItem } from '../src/types';

const main: MenuItem[] = [
  {
    key: 'dashboard',
    icon: 'DashboardOutlined',
    label: '仪表盘',
  },
  {
    key: 'users',
    icon: 'TeamOutlined',
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
    icon: 'UsergroupAddOutlined',
    label: '角色',
    children: [
      {
        key: 'list',
        icon: '',
        label: '列表',
      },
    ],
  },
  {
    key: 'permissions',
    icon: 'UnlockOutlined',
    label: '权限',
    children: [
      {
        key: 'list',
        icon: '',
        label: '列表',
      },
    ],
  },
];

const cms = [
  {
    key: 'posts',
    label: '文章',
    icon: 'DollarOutlined',
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
    icon: 'ClusterOutlined',
  },
  {
    key: 'tags',
    label: '标签',
    icon: 'TagsOutlined',
  },
  {
    key: 'notes',
    label: '笔记',
    icon: 'TagsOutlined',
  },
];

export const menus: MenuItem[] = [
  {
    key: 'main',
    label: '控制台',
    ignore: true,
    children: main,
    icon: 'LaptopOutlined',
  },
  {
    key: 'cms',
    label: '内容管理',
    children: cms,
    icon: 'InboxOutlined',
  },
  {
    key: 'settings',
    label: '设置',
    icon: 'SettingOutlined',
  }
];

const mocks = [
  {
    pattern: '/api/menus',
    handle: (req, res) => {
      const resData = {
        code: "200",
        data: {
            menus,
            total: 0,
            page: 1,
            page_size: 10,
        },
        message: ""
      }
      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(resData))
    },
  },
];

export default mocks;