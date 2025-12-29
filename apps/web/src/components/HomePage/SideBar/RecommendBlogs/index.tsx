'use client';

import { useState, useEffect } from 'react';
import { useMemoizedFn } from 'ahooks';
import { SideBarBlock } from '../SideBarBlock';
import { useFetchNodeList, NodeModel } from '@repo/apis/client';
import { formatDate } from '@repo/utils/client';
import { Image } from '@repo/image';
import { Container } from './index.styled';

export function RecommendBlogs(): JSX.Element {
  const [nodes, setNodes] = useState<NodeModel[]>([]);
  const { fetchNodeList, loading } = useFetchNodeList();

  const init = useMemoizedFn(async () => {
    const res = await fetchNodeList({
      page: 1,
      pageSize: 10,
      orderBy: 'viewed',
      order: 'desc',
    });

    setNodes(res?.nodes);
  });

  useEffect(() => {
    init();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <SideBarBlock title='推荐文章'>
      <Container>
        {loading && (
          <div className='w-full min-h-80 flex items-center justify-center'>
            loading
          </div>
        )}
        <div className="w-full">
          {nodes?.map((node) => {
            return (
              <BlogItem
                key={node?.nid}
                node={node}
              />
            );
          })}
        </div>
      </Container>
    </SideBarBlock>
  );
};

interface BlogItemProps {
  node: NodeModel;
}

function BlogItem({ node }: BlogItemProps): JSX.Element {
  const showThumb = false;

  return (
    <a
      href={`/p/${node?.vid}`}
      className='hover:text-primary transition-colors flex items-center gap-2 mb-2 text-sm'
    >
      {showThumb && (
        <span className="lx-img-w bg-gray-100 rounded-md p-2 flex items-center justify-center h-12 w-12 border border-gray-200">
          <Image
            src={''}
            alt={node?.title || ''}
            width={48}
            height={48}
            className='w-full h-full object-cover'
          />
        </span>
      )}
      <span className="flex flex-col flex-1 justify-between">
        <span>
          {node?.title}
        </span>
        <span className="text-xs text-gray-400">
          {formatDate(node?.createdAt)}
        </span>
      </span>
    </a>
  );
}
