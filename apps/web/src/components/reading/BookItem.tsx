import Image from 'next/image';
import clsx from 'clsx';

import { Node } from '@/utils';

export interface BookItemProps {
  node?: Node;
}

/**
 * 首页文章列表单个
 */
export const BookItem: React.FC<BookItemProps> = ({ node }) => {
  const category = {} as any; // getCategory(node?.data?.taxonomies?.categories?.[0] || '');

  return (
    <div className='w-full'>
      <div className='overflow-hidden rounded-lg hover:shadow-lg hover:shadow-violet-100 hover:border-violet-400 cursor-pointer border border-slate-200 border-solid'>
        <a
          href={`/reading/${node?.data?.book}`}
          className='w-full h-full flex flex-col lg:flex-row items-center'
        >
          {Boolean(node?.data?.thumb) && (
            <div
              className={clsx('oic-img-w relative w-60 h-36 max-w-48 max-h-36 bg-gray-100')}
            >
              <Image
                alt={node?.data?.title || ''}
                src={node?.data?.thumb || ''}
                className={clsx(
                  'absolute block m-auto p-auto top-0 right-0 bottom-0 left-0 object-cover',
                  'max-w-full max-h-full bg-slate-100',
                  'min-w-28 min-h-28'
                )}
                width={180}
                height={140}
              />
            </div>
          )}
          <div className='w-full p-4 bg-white dark:bg-gray-800'>
            <p className='mb-2 text-xl font-medium text-gray-800 dark:text-white'>
              {node?.data?.title}
            </p>
            <div className="flex items-center">
              <p className='text-gray-300 text-md'>
                {category?.name}
              </p>
              <p className='text-gray-300 text-md'>
                <span>开始时间: </span>
                {node?.data?.startedAt}
              </p>
            </div>
            <p className='font-light text-gray-500 dark:text-gray-300 text-md mt-2'>
              {node?.data?.description}
            </p>
            <div className='flex flex-wrap items-center mt-4 justify-starts'>
              {node?.data?.taxonomies?.tags?.map((item) => {
                return (
                  <div
                    key={item}
                    className='mb-1 text-xs mr-2 py-1.5 px-4 text-gray-600 bg-blue-100 rounded-2xl'
                  >
                    {item}
                  </div>
                );
              })}
            </div>
          </div>
        </a>
      </div>
    </div>
  );
};

