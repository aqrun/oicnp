import BookLayout from '../BookList/BookLayout';
import {
  PoetryListPageDataModel,
  ChapterModel,
} from '@repo/apis/server'
import clsx from 'clsx';
import ChapterPoetryDetail from './ChapterPoetryDetail';

export interface PoetryDetailProps {
  catVid?: string;
  poetry?: PoetryListPageDataModel;
  chapters?: ChapterModel[];
}

export default function ArticleDetail({
  catVid,
  poetry,
  chapters,
}: PoetryDetailProps) {
  return (
    <BookLayout
      catVid={catVid}
      hasSideNav={false}
      hasBlogHero={false}
    >
      <>
        <div
          className='flex flex-1 flex-col overflow-auto'
          // style={{
          //   width: 'calc(100% - 22rem)',
          // }}
        >
          <h1 className='text-[1.8rem] lg:text-[2.67rem] mb-[1.3rem] text-slate-800 lg:leading-[2.67rem] lg:text-center'>
            {poetry?.title}
          </h1>
          <div className='node-metas mb-10 text-gray-400 lg:text-center'>
            <a href={`/cat/${poetry?.dynasty}`}>
              <i className='iconfont icon-benshubook122 mr-1' />
              {poetry?.dynasty}
            </a>
            {Boolean(poetry?.authorName) && (
              <span className='ml-4'>
                <i className='iconfont icon-OOjs_UI_icon_userAvatar mr-1' />
                {poetry?.authorName}
              </span>
            )}
            <div className="tags mt-2">
              {poetry?.tags?.split(',').map((item) => {
                return (
                  <span key={item} className="text-sm text-gray-500 mr-2 ml-2 rounded-sm bg-gray-100 p-1">
                    {item}
                  </span>
                );
              })}
            </div>
          </div>
          {poetry?.isBook === '1' ? (
            <ChapterPoetryDetail chapters={chapters} />
          ) : (
            <article
              className={clsx('oic-article-detail prose lg:prose-p max-w-full break-words', {
                'lg:text-center': poetry?.tags?.indexOf('文言文') === -1,
              })}
            >
              {poetry?.content?.split('\n').map((line, index) => (
                <p key={index}>{line}</p>
              ))}
            </article>
          )}
        </div>
        {/* <TableOfContent /> */}
      </>
    </BookLayout>
  );
}