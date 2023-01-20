import React, { useMemo } from 'react';
import { SITE } from '../../../constants';

import { WeChat } from './WeChat';

export interface ShareData {
  icon: string;
  url: string;
}

export interface SocialShareBaseProps {
  url?: string; // 网址，默认使用 window.location.href
  source?: string; // 来源（QQ空间会用到）, 默认读取head标签：<meta name="site" content="http://overtrue" />
  title?: string; // 标题，默认读取 document.title 或者 <meta name="title" content="share.js" />
  origin?: string; // 分享 @ 相关 twitter 账号
  description?: string; // 描述, 默认读取head标签：<meta name="description" content="PHP弱类型的实现原理分析" />
  image?: string; // 图片, 默认取网页中第一个img标签
}

const weiboKey = '';

const SocialShareBase: React.FC<SocialShareBaseProps> = ({
  url,
  source,
  title,
  origin,
  description,
  image,
}) => {
  const win = window as any;
  const links = useMemo(() => {
    const currentUrl = url || win.location.href;
    const currentTitle = title || getMetaContentByName('title') || win.document.title;
    const currentDesc = description || getMetaContentByName('description');

    return [
      {
        icon: 'weibo',
        url: `https://service.weibo.com/share/share.php?url=${currentUrl}&title=${currentTitle}&pic=${image || ''}&appkey=${weiboKey || ''}`,
      },
      {
        icon: 'wechat',
        url: ''
      },
      {
        icon: 'douban',
        url: `http://shuo.douban.com/!service/share?href=${currentUrl}&name=${currentTitle}&text=${currentDesc}&image=${image || ''}&starid=0&aid=0&style=11`,
      },
      {
        icon: 'twitter',
        url: `https://twitter.com/intent/tweet?text=${currentTitle}&url=${currentUrl}&via=${origin || ''}`,
      }
    ];
  }, [url, title, origin, description, image, win.location.href, win.document.title]);

  if (!SITE.socialShare) return null;

  return (
    <div className="social-share-wrapper">
      <div className="social-share">
        {links.map(item => {
          if (item.icon === 'wechat') {
            return <WeChat url={url || ''} key="wechat" />
          }
          return (
            <a
              key={item.icon}
              className={`social-share-icon icon-${item.icon}`}
              target="_blank"
              href={item?.url}
              rel="noreferrer"
            />
          );
        })}
      </div>
    </div>
  );
};

export default SocialShareBase;

function getMetaContentByName(name: string) {
  const metas: NodeListOf<HTMLElement> = document.getElementsByName(name);
  return (metas?.[0] as HTMLMetaElement)?.content || '';
}
