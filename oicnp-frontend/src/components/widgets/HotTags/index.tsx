import React from 'react';

export interface HotTagsProps {

}

export const HotTags: React.FC<HotTagsProps> = () => {
  const tags = [
    { name: '可穿戴', num: 3 },
    { name: '电源', num: 2 },
    { name: '移动电源', num: 2 },
    { name: '手环', num: 2 },
    { name: '摩托车', num: 2 },
    { name: '智能眼镜', num: 2 },
    { name: '隐形眼镜', num: 1 },
    { name: '运动', num: 1 },
    { name: '耳机', num: 1 },
    { name: '癌症', num: 1 },
    { name: '口腔', num: 1 },
    { name: '太阳能', num: 1 },
    { name: '3D打印', num: 1 },
    { name: '裙子', num: 1 },
    { name: '岛国', num: 1 },
    { name: 'Xbox', num: 1 },
    { name: '手柄', num: 1 },
    { name: '游戏', num: 1 },
    { name: '电动车', num: 1 },
    { name: '自行车', num: 1 },
  ];

  return (
    <div className="oic-widget oic-widget-hotTags">
      <h3 className="oic-widget-title">热门标签</h3>
      <div className="oic-hot-tags">
        <div className="oic-inner mr-[-8px]">
          {tags?.map((item)=> {
            return (
              <a
                key={item?.name}
                className="oic-tag-item opacity-80 text-white inline-block px-2
                  hover:opacity-100
                  mr-2 mb-2 text-sm leading-7"
              >
                {item?.name}({item?.num})
              </a>
            );
          })}
        </div>
      </div>
    </div>
  );
}