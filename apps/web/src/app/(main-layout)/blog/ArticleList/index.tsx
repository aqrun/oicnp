import BlogLayout from './BlogLayout';
import { LoadMore } from './LoadMore';
import {
  ArticleItem,
} from '@/components/HomePage';
import {
  DescribeNodeListResponseData,
} from '@repo/apis/server';

export interface ArticleListProps {
  catVid?: string;
  nodeRes?: DescribeNodeListResponseData;
}

export default function ArticleList({
  catVid,
  nodeRes,
}: ArticleListProps) {

  return (
    <BlogLayout
      catVid={catVid}
    >
      <div className='relative flex flex-wrap flex-row gap-2'>
        {(nodeRes?.nodes || [])?.map((item) => {
          return <ArticleItem key={item?.nid} node={item} />;
        })}

        <LoadMore
          catVid={catVid}
        />
      </div>
    </BlogLayout>
  );
}