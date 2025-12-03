import Image from 'next/image';
import { PoetryListPageDataModel, ChapterModel } from '@repo/apis/server';
import {
  Container,
} from './index.styled';

export interface PoemItemProps {
  category?: string;
  record?: PoetryListPageDataModel;
  chapters?: ChapterModel[];
}

/**
 * 诗词列表单个
 */
export function PoemItem({
  category,
  record,
  chapters,
}: PoemItemProps) {
  let contents = record?.content?.split('\n')?.slice(0, 2);

  if ((record?.content?.length || 0) > 100) {
    contents = [record?.content?.slice(0, 40)?.replace(/\n/g, '') || ''];
  }

  if (record?.content === 'book') {
    if (record?.description) {
      contents = [record?.description?.slice(0, 40)?.replace(/\n/g, '') || ''];
    } else if (record?.isBook) {
      contents = [chapters?.[0]?.content?.slice(0, 40)?.replace(/\n/g, ' ') || ''];
    } else {
      contents = ['暂无内容'];
    }
  }

  return (
    <Container className='poem-list-item-w w-1/2 mb-4'>
      <div className='poem-list-item overflow-hidden rounded-lg hover:shadow-lg hover:shadow-violet-100 hover:border-violet-400 cursor-pointer border border-slate-200 border-solid'>
        <a
          href={`/poetry/n/${record?.uuid}`}
          className='block w-full h-full md:flex'
        >
          {Boolean(0) && (
            <Image
              alt={record?.title || ''}
              src='/images/big-news1.jpeg'
              className='object-cover w-full max-h-40 bg-slate-100 md:w-80 md:max-h-full'
              width={180}
              height={180}
            />
          )}
          <div className='w-full p-4 bg-white dark:bg-gray-800 min-h-40'>
            <p className='font-light text-gray-400 text-md'>
              <i className='iconfont icon-benshubook122 mr-1' />
              {category}
              &nbsp;

              {Boolean(record?.authorName) && (
                <span className="ml-2 text-gray-400 font-light">
                  <i className='iconfont icon-OOjs_UI_icon_userAvatar mr-1' />
                  {record?.authorName}
                </span>
              )}
            </p>
            <p className='mb-2 text-xl font-medium text-gray-800 dark:text-white'>
              {record?.title}
            </p>
            <div className='font-light text-gray-800 dark:text-gray-300 text-md'>
              {contents?.map((n, index) => {
                return (
                  <p key={index}>{n}</p>
                );
              })}
            </div>
            <div className='flex flex-wrap items-center mt-4 justify-starts hidden'>
              {record?.tags?.split(',')?.slice(0, 3)?.map((item) => {
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
    </Container>
  );
};

