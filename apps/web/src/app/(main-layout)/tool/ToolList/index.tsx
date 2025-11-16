import ToolLayout from './ToolLayout';
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
    <ToolLayout
      catVid={catVid}
    >
      <div className='relative flex flex-wrap flex-row gap-2'>
        常用工具列表
      </div>
    </ToolLayout>
  );
}