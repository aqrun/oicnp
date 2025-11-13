'use client';

import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { usePersistFn } from '@/utils/common';

export interface HeaderData {
  id: string;
  text: string;
  level: number;
}

export interface TableOfContent {
  _?: string;
}

export const TableOfContent: React.FC<TableOfContent> = () => {
  const [show, setShow] = useState(false);
  const [right, setRight] = useState('-100%');
  const [contentHeight, setContentHeight] = useState('100%');
  const [headers, setHeaders] = useState<HeaderData[]>([]);
  
  const findHeaders = usePersistFn(() => {
    const $article = document.querySelector('.oic-article-detail');

    if (!$article) return;

    const headerData = Array.from($article?.querySelectorAll('h2, h3, h4'))?.map((item: any, index) => {

      if (!item.id) {
        item.setAttribute('id', `toc-header-${index}`);
      }
      return {
        id: item?.id,
        text: item?.innerText,
        level: Number(item?.nodeName?.charAt(1)),
      };
    });

    if (headerData?.length) {
      setHeaders(headerData);
      setTimeout(() => {
        const tocRect = document.querySelector('.oic-toc')?.getBoundingClientRect();
        const width = tocRect?.width || 0;
        const height = tocRect?.height || 0;

        if (window?.innerWidth < 1000) {
          setContentHeight(`${height - 2}px`);
        }
        setRight(`${0 - width - 1}px`);
      }, 500);
    }
  });
  
  useEffect(() => {
    findHeaders();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (!headers?.length) return null;

  return (
    <div
      className={clsx(
        'oic-toc fixed bg-white lg:block lg:left-1/2 top-24 lg:top-28 lg:w-64',
        'lg:ml-[36rem] shadow-md rounded-sm box-content lg:max-h-max',
        'max-h-[80%] border border-gray-200 lg:bg-white transition-all',
      )}
      style={{
        right,
      }}
    >
      <div
        className='absolute top-0 left-[-2.25rem] leading-8 border border-slate-200 lg:hidden bg-white h-9 w-9'
        onClick={() => {
          const width = document.querySelector('.oic-toc')?.getBoundingClientRect().width || 0;
          const currentShow = !show;
          setShow(currentShow);
          setRight(currentShow ? '0' : `${0 - width - 1}px`);
        }}
      >
        <i className='iconfont icon-menu text-4xl text-slate-600 relative top-[-4px]' />
      </div>
      <div
        className={clsx('p-5 overflow-auto')}
        style={{
          height: contentHeight,
        }}
      >
        <ul>
          {headers?.map((item) => {
            const levelClass = item.level === 3 ? 'ml-8' : '';
            return (
              <li
                key={item.id}
                className={`level-${item.level} hover:text-indigo-500 ${levelClass} cursor-pointer text-gray-700 text-sm leading-4 py-1`}
                onClick={() => {
                  document.querySelector(`#${item.id}`)?.scrollIntoView({
                    behavior: 'smooth',
                  });
                }}
              >
                {item?.text}
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
};