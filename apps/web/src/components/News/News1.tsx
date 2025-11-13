import { getCategory, getNewsList } from '@/utils';

export const News1 = () => {
  const newsList = getNewsList();

  return (
    <div className='oic-news1 max-w-screen-xl mx-auto dark:bg-gray-800 text-gray-100'>
      <div className='grid grid-cols-1 gap-4 lg:grid-cols-4 sm:grid-cols-2'>
        {newsList?.slice(0, 4)?.map((item, index) => {
          const category = getCategory(item?.data?.taxonomies?.categories);
          return (
            <div
              key={item?.data?.title}
              className='relative flex items-end justify-start w-full text-left bg-center bg-cover h-96 bg-gray-500'
              // style='background-image: url("https://source.unsplash.com/random/240x320");'
              // style={{
              //   backgroundImage: `url("https://source.unsplash.com/random/263x384?${
              //     imageKeyWords?.[index + 1]
              //   }")`,
              // }}
              style={{
                backgroundImage: `url("/images/big-news${index + 2}.jpeg")`,
              }}
            >
              <div className='absolute top-0 bottom-0 left-0 right-0 bg-gradient-to-b via-transparent from-gray-900 to-gray-900'></div>
              <div className='absolute top-0 left-0 right-0 flex items-center justify-between mx-5 mt-3'>
                <a
                  rel='noopener noreferrer'
                  href={`/category/${category?.vid}`}
                  className='px-3 py-2 text-xs font-semibold tracki uppercase text-gray-100 bgundefined'
                >
                  {category?.name}
                </a>
                <div
                  className='flex flex-col justify-start text-center text-gray-100'
                  data-date={item?.data?.date?.toISOString()}
                >
                  <span className='text-3xl font-semibold leadi tracki'>
                    {(item?.data?.date?.getMonth() || 0) + 1}/
                    {item?.data?.date?.getDate()}
                  </span>
                  <span className='leadi uppercase'>
                    {item?.data?.date?.getFullYear() || 0}
                  </span>
                </div>
              </div>
              <h2 className='z-10 p-5'>
                <a
                  rel='noopener noreferrer'
                  href={`/n/${item?.data?.slug}`}
                  className='font-medium text-md hover:underline text-gray-100 hover:text-violet-500'
                >
                  {item?.data?.title}
                </a>
              </h2>
            </div>
          );
        })}
      </div>
    </div>
  );
};
