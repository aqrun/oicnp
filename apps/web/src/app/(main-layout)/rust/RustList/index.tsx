import RustLayout from './RustLayout';
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
    <RustLayout
      catVid={catVid}
    >
      <div className='relative flex flex-wrap flex-row gap-2'>
        rust
      </div>
    </RustLayout>
  );
}