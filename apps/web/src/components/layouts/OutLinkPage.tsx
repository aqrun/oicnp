'use client';

import React from 'react';
import { useSearchParams } from 'next/navigation';
import {
  siteConfig,
} from '@/constant/config';

export interface OutLinkPageProps {
  url?: string;
}

/**
 * 外部链接盅提示页
 */
export default function OutLinkPage({
  url,
}: OutLinkPageProps) {
  // 获取URL参数 target
  const params = useSearchParams();
  const target = params.get('target');
  let link = target ? decodeURIComponent(target) : url;

  if (url) {
    link = decodeURIComponent(url);
  }
  
  console.log('target-->', target, params);
  return (
    <div className="w-screen h-screen flex justify-center bg-gray-200">
      <div className='out-link-page mt-24 ml-auto mr-auto max-w-xl'>
        <div className="logo mb-2">
          <a href={siteConfig.url} className="flex items-center gap-2 font-medium text-3xl">
            <img src='/favicon/logo112501.svg' alt='logo' width={40} height={40} />
            <span>{siteConfig.title}</span>
          </a>
        </div>
        <div className="p-6 bg-white rounded-lg">
          <h4 className="mb-2">即将离开灵犀纪</h4>
          <p>您即将离开灵犀纪，请注意您的账号和财产安全。</p>
          <p className="mt-2">{link}</p>
          <div className="border-t border-gray-200 pt-4 mt-4 flex justify-end">
            <a
              href={link}
              rel='noopener noreferrer'
              className='px-4 py-2 bg-fuchsia-700 text-white rounded-md hover:bg-fuchsia-800 transition-colors'
            >
              继续访问
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}