import { NextRequest, NextResponse } from 'next/server';

import { HOME_PAGE_SIZE } from '@/constant';
import { getNodes } from '@/utils';

/*
export const generateStaticParams = () => {
  const all_nodes = getAllNodes();
  const total = Math.ceil(all_nodes?.length / HOME_PAGE_SIZE);

  const pages = [];

  for (let i = 0; i < total; i ++) {
    pages.push(i + 1);
  }

  return pages?.map((page) => {
    return {
      page: `${page}`,
    };
  });
};
*/

export async function GET(req: NextRequest, ctx: { params: { page: string } }) {
  // const ReactDOMServer = (await import('react-dom/server')).default

  const all_nodes = getNodes({
    orderBy: 'date',
  });
  const page = Number(ctx?.params?.page || 2);
  const pageSize = HOME_PAGE_SIZE;
  const startIndex = (page - 1) * pageSize;
  const endIndex = startIndex + pageSize;
  const results = [];
  const nodes = all_nodes?.slice(startIndex, endIndex);

  for (let i = 0; i < nodes.length; i++) {
    const item = nodes?.[i];
    // const content = item?.content;
    delete item.content;
    // delete (item as any)?.orig;
    // const Content = await parseMdx(content || '');
    // const html = ReactDOMServer.renderToString(React.createElement(Content));
    // (item as any).content = html;
    results.push(item);
  }

  return NextResponse.json({
    code: '200',
    data: {
      nodes: results,
    },
    page,
    pageSize,
  });
}
