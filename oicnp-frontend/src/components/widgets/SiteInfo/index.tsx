import React from 'react';

export interface SiteInfoProps {

}

export const SiteInfo: React.FC<SiteInfoProps> = () => {

  const infos = [
    { name: '日志总数', num: 49 },
    { name: '评论总数', num: 30 },
    { name: '标签总数', num: 34 },
    { name: '页面总数', num: 11 },
    { name: '分类总数', num: 28 },
    { name: '链接总数', num: 14 },
    { name: '用户总数', num: 6138 },
    { name: '最后更新', num: '2023-12-01' },
  ];

  return (
    <div className="oic-widget oic-widget-siteInfo">
      <h3 className="oic-widget-title">网站统计</h3>
      <ul
        className="py-3 px-5 leading-loose text-gray-500 flex flex-wrap"
      >
        {infos?.map((item) => {
          return (
            <li
              key={item?.name}
              className="w-1/2"
            >
              <span
                className="text-gray-800"
              >
                {item?.name}：
              </span>
              <span>{item?.num}</span>
            </li>
          );
        })}
      </ul>
    </div>
  );
};
