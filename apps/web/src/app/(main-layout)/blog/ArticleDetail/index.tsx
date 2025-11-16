import BlogLayout from '../ArticleList/BlogLayout';
import { formatDate } from '@/utils/common';
import {
  NodeModel,
} from '@repo/apis/server';

export interface ArticleDetailProps {
  catVid?: string;
  node?: NodeModel;
  content?: string;
  hasArticleMeta?: boolean;
}

export default function ArticleDetail({
  catVid,
  node,
  hasArticleMeta = true,
  content
}: ArticleDetailProps) {
  return (
    <BlogLayout
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
            {node?.title}
          </h1>
          {hasArticleMeta && (
            <div className='node-metas mb-10 text-gray-400 lg:text-center'>
              <a href={`/cat/${node?.categories?.[0]?.catVid}`}>
                <i className='iconfont icon-benshubook122 mr-1' />
                {node?.categories?.[0]?.catName}
              </a>
              <span className='ml-4'>
                <i className='iconfont icon-date mr-1' />
                {formatDate(node?.createdAt)}
              </span>
            </div>
          )}
          <article
            className='oic-article-detail prose lg:prose-p max-w-full break-words'
            dangerouslySetInnerHTML={{
              __html: content || '',
            }}
          />
        </div>
        {/* <TableOfContent /> */}
      </>
    </BlogLayout>
  );
}