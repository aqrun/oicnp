'use client';

import { useMemo } from 'react';
import { useBlogStore } from '../useBlogStore';
import { NodeModel } from '@repo/apis/client';
import {
  ArticleItem,
} from '@/components/HomePage';

/**
 * 动态加载的数据列表
 */
export default function DynamicNodeList(): JSX.Element{
  const nodeResList = useBlogStore((state) => state.nodeResList);

  const allNodes = useMemo(() => {
    return nodeResList?.reduce((acc, curr) => {
      return [...acc, ...(curr?.nodes || [])];
    }, [] as NodeModel[]);
  }, [nodeResList]);

  return (
    <>
      {allNodes?.map((item) => {
        return <ArticleItem key={item?.nid} node={item} />;
      })}
    </>
  );
};
