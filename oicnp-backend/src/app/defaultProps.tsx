import {
  ChromeFilled,
  CrownFilled,
  SmileFilled,
  TabletFilled,
} from '@ant-design/icons';

const defaultProps = {
  route: {
    path: '/',
    routes: [
      {
        path: '/welcome',
        name: '欢迎',
        icon: <SmileFilled />,
        component: './Welcome',
      },
      {
        path: '/admin',
        name: '内容管理',
        icon: <CrownFilled />,
        access: 'canAdmin',
        component: './Admin',
        routes: [
          {
            path: '/admin/nodes',
            name: '文章',
            icon: 'https://gw.alipayobjects.com/zos/antfincdn/upvrAjAPQX/Logo_Tech%252520UI.svg',
            component: './Welcome',
          },
          {
            path: '/admin/categories',
            name: '分类',
            icon: <CrownFilled />,
            component: './Welcome',
          },
          {
            path: '/admin/tags',
            name: '标签',
            icon: <CrownFilled />,
            component: './Welcome',
          },
        ],
      },
      {
        name: '列表页',
        icon: <TabletFilled />,
        path: '/list',
        component: './ListTableList',
        routes: [
          {
            path: '/list/sub-page',
            name: '列表页面',
            icon: <CrownFilled />,
            routes: [
              {
                path: 'sub-sub-page1',
                name: '一一级列表页面',
                icon: <CrownFilled />,
                component: './Welcome',
              },
              {
                path: 'sub-sub-page2',
                name: '一二级列表页面',
                icon: <CrownFilled />,
                component: './Welcome',
              },
              {
                path: 'sub-sub-page3',
                name: '一三级列表页面',
                icon: <CrownFilled />,
                component: './Welcome',
              },
            ],
          },
          {
            path: '/list/sub-page2',
            name: '二级列表页面',
            icon: <CrownFilled />,
            component: './Welcome',
          },
          {
            path: '/list/sub-page3',
            name: '三级列表页面',
            icon: <CrownFilled />,
            component: './Welcome',
          },
        ],
      },
      {
        path: 'https://ant.design',
        name: 'Ant Design 官网外链',
        icon: <ChromeFilled />,
      },
    ],
  },
  location: {
    pathname: '/',
  },
  // appList: [
  //   {
  //     icon: 'https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg',
  //     title: 'Ant Design',
  //     desc: '杭州市较知名的 UI 设计语言',
  //     url: 'https://ant.design',
  //   },
  //   {
  //     icon: 'https://gw.alipayobjects.com/zos/antfincdn/FLrTNDvlna/antv.png',
  //     title: 'AntV',
  //     desc: '蚂蚁集团全新一代数据可视化解决方案',
  //     url: 'https://antv.vision/',
  //     target: '_blank',
  //   }
  // ],
};

export default defaultProps;
