'use client';

import { useEffect } from 'react';
import { useFetchTags } from '@repo/apis/client';
import { useMemoizedFn } from 'ahooks';
import { useTagsStore } from './useTagsStore';
import { TagsContainer } from './index.styled';

export function Tags() {
  const { fetchTags } = useFetchTags();
  const tagsRes = useTagsStore((state) => state.tagsRes);
  const setState = useTagsStore.setState;

  const init = useMemoizedFn(async () => {
    const res = await fetchTags({
      page: 1,
      pageSize: 20,
    });

    setState({
      tagsRes: res,
    });
  });

  useEffect(() => {
    init();
  }, []);

  return (
    <div className='pb-4 card-base border border-slate-200 mt-4 rounded-lg hover:shadow-md'>
      <div className='transition before:absolute before:bg-[var(--primary)] before:rounded-md before:w-1 font-bold before:h-4 dark:text-neutral-100 relative text-lg text-neutral-900 before:left-[-16px] before:top-[8.5px] mb-2 ml-8 mt-4'>
        标签
      </div>
      <div className='overflow-hidden collapse-wrapper px-4'>
        <TagsContainer className='flex flex-wrap'>
          {tagsRes?.tags?.slice(0, 20)?.map((item) => {
            return (
              <a
                href={`/tag/${item?.tagVid}`}
                key={item?.tagId}
                aria-label='View all posts with the Blogging tag'
                className='text-sm'
                title={item?.tagName}
              >
                <span className="lx-tags-item truncate">
                  {item?.tagName}
                </span>
              </a>
            );
          })}
        </TagsContainer>
      </div>
    </div>
  );
}