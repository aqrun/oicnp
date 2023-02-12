import React from 'react';

export interface BloodRecommendProps {

}

export const BloodRecommend: React.FC<BloodRecommendProps> = () => {

  return (
    <div className="oic-widget oic-widget-bloodRecommend">
      <a
        className="rounded-md px-5 pb-4
          block text-purple-300 border border-solid border-transparent
          transition-colors duration-300 hover:border-purple-400"
      >
        <div
          className="text-white px-4 py-2 text-xs bg-purple-300 inline-block mt-[-1px]"
        >
          吐血推荐
        </div>
        <h3 className="text-lg mt-3">Gentoo Linux</h3>
        <div className="mt-3 text-gray-400 text-sm">
          Gentoo Linux是一套通用的、快捷的、完全免费的Linux发行版，它面向开发人员和网络职业人员。
          与其他发行不同的是，Gentoo Linux拥有一套先进的包管理系统叫作Portage。
        </div>
      </a>
    </div>
  );
}