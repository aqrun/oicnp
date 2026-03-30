import ToolLayout from './ToolLayout';
import { ToolItem, ToolCategories } from '@/content/tools';
import ToolItemWidget from './ToolItemWidget';
import {
  ListBlockTitle,
} from '@/components';
import { ToolList } from './index.styled';

export interface ArticleListProps {
  catVid?: string;
  title?: string;
  toolList?: ToolItem[];
  toolCategories?: ToolCategories[];
}

export default function ArticleList({
  catVid,
  title,
  toolList,
  toolCategories,
}: ArticleListProps) {

  return (
    <ToolLayout
      catVid={catVid}
      toolCategories={toolCategories}
    >
      <ListBlockTitle
        title={title || ''}
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {toolList?.map((item) => {
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