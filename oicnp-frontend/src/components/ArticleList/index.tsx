import React, { useMemo } from 'react';
import { Node } from '../../typings';
import { ArticleListItem } from '../ArticleListItem';
import { Paginator } from '../Paginator';
import { useMemoizedFn } from 'ahooks';

export interface ArticleList {
  allBlogs: Node[];
  page?: number;
  pageSize?: number;
  totalCount?: number;
}

export const ArticleList: React.FC<ArticleList> = ({
  allBlogs,
  page = 1,
  pageSize = 10,
  totalCount = 0,
}) => {
  const pageCount = useMemo(() => {
    return Math.ceil(totalCount / pageSize);
  }, [totalCount, pageSize]);

  const hrefBuilder = useMemoizedFn((
    pageIndex: number,
    pageCount: number,
    selectedPage: number
  ) => {
    return pageIndex === 1 ? '/' : `/page/${pageIndex}`;
  });

  const pagerClickHandle = useMemoizedFn((e) => {
    const current_page = e?.nextSelectedPage ?? 0;
    location.href = current_page
      ? `/page/${current_page + 1}`
      : '/';
  });

  return (
    <div className="article-list">
      {allBlogs.map((item) => {
        const tags = item?.tags?.map?.(i => i.name);
        return (
          <ArticleListItem
            key={item.nid}
            title={item.title}
            url={`/blog/${item.nid}/${item.vid}`}
            excerpt={item?.nodeBody?.summary}
            tags={tags}
            date={item.createdAt}
          />
        );
      })}

      <Paginator
        initialPage={page - 1}
        pageCount={pageCount}
        hrefBuilder={hrefBuilder}
        onClick={pagerClickHandle}
      />
    </div>
  );
};
