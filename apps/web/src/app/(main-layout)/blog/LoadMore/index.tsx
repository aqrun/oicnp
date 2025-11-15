'use client';

import dynamic from 'next/dynamic';
import { useEffect, useMemo } from 'react';
import { useBlogStore } from '../useBlogStore';
import { useFetchNodeList } from '@repo/apis/client';
import { useMemoizedFn } from 'ahooks';
import { nextTick } from '@repo/utils/client';
import { NodeModel } from '@repo/apis/client';
import { LoadMoreContainer } from './index.styled';

const ArticleItem = dynamic(() => import('../../../../components/HomePage/ArticleItem').then((mod) => mod.ArticleItem), {
  ssr: false,
});

export interface LoadMoreProps {

}

/**
 * 加载更多组件
 */
export const LoadMore = () => {
  const {
    fetchNodeList,
  } = useFetchNodeList();
  const pager = useBlogStore((state) => state.pager);
  const nodeResList = useBlogStore((state) => state.nodeResList);
  const setState = useBlogStore.setState;

  const allNodes = useMemo(() => {
    return nodeResList?.reduce((acc, curr) => {
      return [...acc, ...(curr?.nodes || [])];
    }, [] as NodeModel[]);
  }, [nodeResList]);

  const fetchNodeListData = useMemoizedFn(async () => {
    const page = pager?.page || 1;

    setState({
      loading: true,
    });

    const res = await fetchNodeList({
      page,
      pageSize: pager.pageSize,
    });

    if (res?.nodes?.length) {
      setState({
        loading: false,
        pager: {
          ...pager,
          page,
          total: res?.total || 0,
        },
        nodeResList: [...(nodeResList || []), res],
      });
    }
  });

  const handleLoad = useMemoizedFn(async () => {
    console.log('load9999')
    const page = (pager?.page || 1) + 1;
    const totalPage = Math.ceil((pager.total - pager?.pageSize) / pager.pageSize);

    if (pager.total && page > totalPage) {
      return;
    }

    setState({
      pager: {
        ...pager,
        page,
      },
    });
    await nextTick();
    await fetchNodeListData();
  });

  const init = useMemoizedFn(async () => {
    setState({
      pager: {
        ...pager,
        page: 1,
      },
      nodeResList: [],
    });
  });

  useEffect(() => {
    init();
  }, []);

  return (
    <>
      {allNodes?.map((item) => {
        return <ArticleItem key={item?.nid} node={item} />;
      })}

      <LoadMoreContainer>
        <div
          className="text-center text-gray-500"
          onClick={handleLoad}
        >
          加载更多...
        </div>
      </LoadMoreContainer>
    </>
  );
};