'use client';

import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { usePersistFn } from '@/utils/common';

export interface HeaderData {
  id: string;
  text: string;
  level: number;
}

export interface TableOfContent2Props {
  _?: string;
}

export const TableOfContent2: React.FC<TableOfContent2Props> = () => {
  const [headers, setHeaders] = useState<HeaderData[]>([]);
  
  const findHeaders = usePersistFn(() => {
    const $article = document.querySelector('.oic-article-detail');

    if (!$article) return;

    const headerData = Array.from($article?.querySelectorAll('h2, h3, h4'))?.map((item: Element, index) => {
      const headerElement = item as HTMLElement;
      if (!headerElement.id) {
        headerElement.setAttribute('id', `toc-header-${index}`);
      }
      return {
        id: headerElement.id,
        text: headerElement.innerText,
        level: Number(headerElement.nodeName?.charAt(1)),
      };
    });

    if (headerData?.length) {
      setHeaders(headerData);
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
        'oic-toc2 bg-white lg:block px-[1rem] py-[1rem]',
        'rounded-lg box-content mb-4',
        'border border-gray-200 lg:bg-white transition-all',
      )}
    >
      <div
        className={clsx('overflow-auto')}
      >
        <ul>
          {headers?.map((item) => {
            let levelClass = '';
            if (item.level === 3) {
              levelClass = 'ml-4';
            }
            if (item.level === 4) {
              levelClass = 'ml-8';
            }
            if (item.level === 5) {
              levelClass = 'ml-12';
            }
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
