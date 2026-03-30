import ToolLayout from '../ToolList/ToolLayout';
import { ToolItem, ToolCategories } from '@/content/tools';
import ToolItemWidget from '../ToolList/ToolItemWidget';
import {
  ListBlockTitle,
} from '@/components';
import { ToolList } from '../ToolList/index.styled';

export interface ArticleListProps {
  catVid?: string;
  allTools?: ToolItem[];
  toolCategories?: ToolCategories[];
}

export default function ArticleList({
  catVid,
  allTools,
  toolCategories,
}: ArticleListProps) {
  return (
    <ToolLayout
      catVid={catVid}
      toolCategories={toolCategories}
    >
      <ListBlockTitle
        title="常用推荐"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="React UI"
        moreLink="/tool/t/react-ui"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'static-site-generator')?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="图表 / 可视化"
        moreLink="/tool/t/charts"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'charts')?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="表单与校验"
        moreLink="/tool/t/forms"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'forms')?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="Headless CMS"
        moreLink="/tool/t/headless-cms"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'headless-cms')?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="Markdown / MDX"
        moreLink="/tool/t/markdown-mdx"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'markdown-mdx')?.slice(0, 6)?.map((item) => {
          return (
            <ToolItemWidget
              key={item?.name}
              record={item}
            />
          );
        })}
      </ToolList>
      <ListBlockTitle
        title="AI 应用"
        moreLink="/tool/t/ai-apps"
      />
      <ToolList className='relative flex flex-wrap flex-row'>
        {allTools?.filter((item) => item?.category === 'ai-apps')?.slice(0, 6)?.map((item) => {
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