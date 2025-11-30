import { Metadata } from 'next';
import { redirect } from 'next/navigation';
import { siteConfig } from '@/constant/config';
import ToolList from '../../ToolList';
import { MainLayout } from '@/components/layouts';
import { ALL_TOOLS, TOOL_CATEGORIES } from '@/content/tools';

export const metadata: Metadata = {
  title: '常用工具|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export interface ToolsListPageProps {
  params: {
    catVid: string;
  };
}

export default async function ToolsListPage({
  params,
}: ToolsListPageProps) {
  const { catVid } = await params;

  if (catVid === 'all') {
    return redirect('/tool');
  }

  const toolList = ALL_TOOLS.filter((item) => item?.category === catVid);
  const title = TOOL_CATEGORIES.find((item) => item?.id === catVid)?.name;
  
  return (
    <MainLayout
      activeMenuId='tool'
    >
      <ToolList
        catVid={catVid}
        title={title}
        toolList={toolList}
        toolCategories={TOOL_CATEGORIES}
      />
    </MainLayout>
  );
}
