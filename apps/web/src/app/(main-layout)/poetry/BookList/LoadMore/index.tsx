'use client';

import { useEffect, useMemo } from 'react';
import { useBookStore, defaultState } from '../../useBookStore';
import {
  ChapterModel,
  PoetryListPageDataModel,
  useFetchPoetryListWithChapters,
} from '@repo/apis/client';
import { useMemoizedFn } from 'ahooks';
import { nextTick } from '@repo/utils/client';
import { PoemItem } from '../../BookHome/PoemItem';
import {
  DescribePoetryListWithChaptersRequestParams,
} from '@repo/apis/client';
import { LoadMoreContainer } from './index.styled';
import { BOOK_CATEGORIES } from '@/content/books/base';

export interface LoadMoreProps {
  catId?: string;
  needLoadMore?: boolean;
}

/**
 * 加载更多组件
 */
export const LoadMore = ({
  catId,
  needLoadMore = true,
}: LoadMoreProps) => {
  const {
    fetchPoetryListWithChapters,
  } = useFetchPoetryListWithChapters();
  const pager = useBookStore((state) => state.pager);
  const poetryResList = useBookStore((state) => state.poetryResList);
  const hasMore = useBookStore((state) => state.hasMore);
  const loading = useBookStore((state) => state.loading);
  const setState = useBookStore.setState;

  const category = BOOK_CATEGORIES.find((item) => item?.id === catId);

  const allPoetry = useMemo(() => {
    return poetryResList?.reduce((acc, curr) => {
      return [...acc, ...(curr?.entry?.poetry_list || [])];
    }, [] as PoetryListPageDataModel[]);
  }, [poetryResList]);
  const allChapters = useMemo(() => {
    return poetryResList?.reduce((acc, curr) => {
      return [...acc, ...(curr?.entry?.chapter_list || [])];
    }, [] as ChapterModel[]);
  }, [poetryResList]);

  const fetchPoetryListData = useMemoizedFn(async () => {
    const page = pager?.page || 1;

    setState({
      loading: true,
    });

    const params: DescribePoetryListWithChaptersRequestParams = {
      page,
      pageSize: pager.pageSize,
      tags: category?.tags?.join(','),
      chapterAmount: 5,
      order: 'asc',
      orderBy: 'id',
    };

    if (category?.dynasty) {
      params.dynasty = category?.dynasty;
    }

    const res = await fetchPoetryListWithChapters(params);

    if (res?.entry?.poetry_list?.length) {
      setState({
        loading: false,
        pager: {
          ...pager,
          page,
          total: res?.entry?.total || 0,
        },
        poetryResList: [...(poetryResList || []), res],
      });
    } else {
      setState({
        loading: false,
        hasMore: false,
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
    await fetchPoetryListData();
  });

  const init = useMemoizedFn(async () => {
    setState({
      ...defaultState,
    });
    nextTick();
    // handleLoad();
  });

  useEffect(() => {
    init();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      {allPoetry?.map((item) => {
        const bookChapters = allChapters?.filter((chapter) => {
          return chapter?.poetryId === item?.id;
        });

        return (
          <PoemItem
            key={item?.id}
            category={category?.name}
            record={item}
            chapters={bookChapters}
          />
        );
      })}

      {hasMore && needLoadMore && (
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