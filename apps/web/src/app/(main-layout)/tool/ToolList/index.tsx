import ToolLayout from './ToolLayout';
import {
  DescribeNodeListResponseData,
} from '@repo/apis/server';
import { ALL_TOOLS } from '@/content/tools';
import ToolItemWidget from './ToolItemWidget';
import { ToolList } from './index.styled';

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
      <ToolList className='relative flex flex-wrap flex-row'>
        {ALL_TOOLS?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
    </ToolLayout>
  );
}