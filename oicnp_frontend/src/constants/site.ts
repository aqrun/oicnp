export const SITE = {
  name: 'AQRUN',

  // Site settings 配置站点
  title: '子十的独立博客',
  description: '热爱技术。很高兴能在这里与你分享我对技术和生活的思考。',
  keyword: 'AQRUN,子十,AQRUN的独立博客,子十的博客,前端,javascript,vuejs,nodejs,RUST,PHP,java,react,骑行,跑步,爬山',
  url: 'http://aqrun.com', // your host

  footer: {
    since: 2014,
  },

  // Author 配置博主信息
  author: '子十',
  nickname: '子十',
  bio: '天行健，君子以自强不息。<br/>地势坤，君子以厚德载物。',
  avatar: '/assets/img/profile.png?v=0.1',

  // Search
  search: true,
  // Night mode
  nightMode: true,

  // Share
  socialShare: true,
  socialShareItems: ['wechat', 'weibo', 'douban', 'twitter'],
  // theme color 主题皮肤
  themeColor: 'default', // pink or default

  // Post header background patterns (when the post no cover): circuitBoard, overlappingCircles, food, glamorous, ticTacToe, seaOfClouds
  postPatterns: 'circuitBoard',

  // SNS settings 配置社交网站
  // url: email, weibo, zhihu, twitter, instagram, juejin, github, douban, facebook, dribble, uicn, jianshu, medium, linkedin
  sns: [
    { name: 'weibo', url: '//weibo.com/aqrun' },
    { name: 'jianshu', url: '//www.jianshu.com/u/0b2ee2adc013' },
    { name: 'gitee2', url: '//gitee.com/aqrun' },
    { name: 'github', url: '//github.com/aqrun' },
  ],

  // Permalink
  // See: https://github.com/kaeyleo/jekyll-theme-H2O/issues/35
  permalink: '/:year/:month/:day/:title.html',
  pageSize: 5,
};

export type SiteConfig = typeof SITE;