import Image from 'next/image';

import { getCategory, getNewsList } from '@/utils';

export const BigNews = () => {
  const news_list = getNewsList();
  const node = news_list?.[0];
  const category = getCategory(node?.data?.taxonomies?.categories?.[0] || '');
  return (
    <section className='oic-latest-news-section'>
      <div className='layout'>
        {/* <!-- Content --> */}
        <div className='grid justify-items-stretch'>
          <a
            href={`/n/${node?.data?.slug}`}
            className='relative flex h-[500px] flex-col gap-4 rounded-md px-4 py-8 [grid-area:1/1/2/2] md:p-0 md:[grid-area:1/1/2/4] hover:text-violet-500'
          >
            <div className='absolute bottom-12 left-8 z-20 flex w-56 max-w-[464px] flex-col items-start rounded-md bg-white p-6 sm:w-full md:bottom-[10px] md:left-[10px]'>
              <div className='mb-4 rounded-md bg-[#f2f2f7] px-2 py-1.5'>
                <p className='text-sm font-semibold text-[#6574f8]'>
                  {category?.name}
                </p>
              </div>
              <p className='mb-4 max-w-xs text-xl font-bold md:text-2xl text-gray-800 hover:text-violet-500'>
                {node?.data?.title}
              </p>
              <p className='mb-4 text-base md:text-base text-gray-500'>
                {node?.data?.description}
              </p>
              <div className='flex flex-col text-sm text-gray-500 lg:flex-row'>
                <p>子十</p>
                <p className='mx-2 hidden lg:block'>-</p>
                <p>阅读时间：6分钟</p>
              </div>
            </div>
            <Image
              src='/images/big-news1.jpeg'
              alt=''
              className='inline-block h-full w-full object-cover'
              width={1100}
              height={500}
              unoptimized
              // style={{
              //   backgroundImage:
              //     'url("https://source.unsplash.com/random/1100x500?mountain")',
              // }}
            />
          </a>
        </div>
      </div>
    </section>
  );
};
