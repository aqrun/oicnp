import Image from 'next/image';

import { getCategory, Node, formatDate } from '@/utils';

export interface ArticleItemProps {
  node?: Node;
}

/**
 * 首页文章列表单个
 */
export const ArticleItem: React.FC<ArticleItemProps> = ({ node }) => {
  const category = getCategory(node?.data?.taxonomies?.categories?.[0] || '');
  return (
    <div className='w-full'>
      <div className='overflow-hidden rounded-lg hover:shadow-lg hover:shadow-violet-100 hover:border-violet-400 cursor-pointer border border-slate-200 border-solid'>
        <a
          href={`/n/${node?.data?.slug}`}
          className='block w-full h-full md:flex'
        >
          {Boolean(node?.data?.thumb) && (
            <Image
              alt={node?.data?.title || ''}
              src='/images/big-news1.jpeg'
              className='object-cover w-full max-h-40 bg-slate-100 md:w-80 md:max-h-full'
              width={180}
              height={180}
            />
          )}
          <div className='w-full p-4 bg-white dark:bg-gray-800'>
            <p className='font-light text-gray-400 text-md'>
              <i className='iconfont icon-benshubook122 mr-1' />
              {category?.name}
              &nbsp;
              <span className="ml-2 text-gray-400 font-light">
                <i className='iconfont icon-date mr-1' />
                {formatDate(node?.data?.date)}
              </span>
            </p>
            <p className='mb-2 text-xl font-medium text-gray-800 dark:text-white'>
              {node?.data?.title}
            </p>
            <p className='font-light text-gray-400 dark:text-gray-300 text-md'>
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

