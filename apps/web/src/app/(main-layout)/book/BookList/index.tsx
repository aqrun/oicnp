import BookLayout from './BookLayout';
import {
  DescribeNodeListResponseData,
} from '@repo/apis/server';

export interface ArticleListProps {
  catVid?: string;
  tagVid?: string;
  nodeRes?: DescribeNodeListResponseData;
}

export default function ArticleList({
  catVid,
  tagVid,
  nodeRes,
}: ArticleListProps) {

  return (
    <BookLayout
      catVid={catVid}
    >
      <div className='relative flex flex-wrap flex-row gap-2'>
        阅读列表
      </div>
    </BookLayout>
  );
}