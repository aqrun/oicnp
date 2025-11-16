'use client';

import { useEffect, useMemo } from 'react';
import { useBlogStore } from '../../useBlogStore';
import { useFetchNodeList } from '@repo/apis/client';
import { useMemoizedFn } from 'ahooks';
import { nextTick } from '@repo/utils/client';
import {
  NodeModel,
  DescribeNodeListRequestParams,
} from '@repo/apis/client';
import {
  ArticleItem,
} from '@/components/HomePage/ArticleItem';
import { LoadMoreContainer } from './index.styled';

export interface LoadMoreProps {
  catVid?: string;
}

/**
 * 加载更多组件
 */
export const LoadMore = ({
  catVid,
}: LoadMoreProps) => {
  const {
    fetchNodeList,
  } = useFetchNodeList();
  const pager = useBlogStore((state) => state.pager);
  const nodeResList = useBlogStore((state) => state.nodeResList);
  const hasMore = useBlogStore((state) => state.hasMore);
  const loading = useBlogStore((state) => state.loading);
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

    const params: DescribeNodeListRequestParams = {
      page,
      pageSize: pager.pageSize,
    };

    if (catVid && catVid !== 'all') {
      params.categoryVids = catVid;
    }

    const res = await fetchNodeList(params);

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
    if (loading) {
      return;
    }

    const page = (pager?.page || 1) + 1;
    const totalPage = Math.ceil((pager.total - pager?.pageSize) / pager.pageSize);

    if (pager.total && page > totalPage) {
      setState({
        hasMore: false,
      });
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

      {hasMore && (
        <LoadMoreContainer className='w-full'>
          <div
            className="text-center text-gray-500 cursor-pointer hover:text-purple-700"
            onClick={handleLoad}
          >
            点击加载更多...
          </div>
        </LoadMoreContainer>
      )}
    </>
  );
};