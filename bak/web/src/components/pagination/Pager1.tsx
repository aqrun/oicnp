export interface PagerProps {
  page: number;
  pageSize: number;
  total: number;
  baseUrl?: string;
}

export const Pager1: React.FC<PagerProps> = ({
  page,
  pageSize,
  total,
  baseUrl,
}) => {
  const pages = [];
  const totalPage = Math.ceil(total / pageSize);

  for (let i = 0; i < totalPage; i++) {
    pages.push(i + 1);
  }

  if (!pages?.length || pages?.length <= 1) {
    return null;
  }

  return (
    <div className='flex justify-center flex-wrap space-x-1 text-slate-800 mt-6'>
      {page > 1 && (
        <a
          href={`${baseUrl}/${page - 1}`}
          title='previous'
          type='button'
          className='cursor-pointer inline-flex items-center justify-center w-8 h-8 py-0 border rounded-md hover:shadow-md bg-white border-gray-200 hover:text-violet-600 hover:border-violet-600'
        >
          <svg
            viewBox='0 0 24 24'
            stroke='currentColor'
            strokeWidth='2'
            fill='none'
            strokeLinecap='round'
            strokeLinejoin='round'
            className='w-4'
          >
            <polyline points='15 18 9 12 15 6'></polyline>
          </svg>
        </a>
      )}
      {pages?.map((i) => {
        const activeClass =
          page === i
            ? 'shadow-md bg-white text-violet-600 border-violet-600'
            : '';
        return (
          <a
            href={`${baseUrl}/${i}`}
            key={i}
            type='button'
            title={`第 ${i} 页`}
            className={`mb-1 cursor-pointer inline-flex items-center justify-center w-8 h-8 text-sm font-semibold border
              rounded bg-white text-gray-500 ${activeClass} hover:shadow-md hover:text-violet-600 hover:border-violet-600`}
          >
            {i}
          </a>
        );
      })}
      {page < totalPage && (
        <a
          href={`${baseUrl}/${page + 1}`}
          title='next'
          type='button'
          className='cursor-pointer inline-flex items-center justify-center w-8 h-8 py-0 border rounded-md hover:shadow-md bg-white border-gray-200 hover:text-violet-600 hover:border-violet-600'
        >
          <svg
            viewBox='0 0 24 24'
            stroke='currentColor'
            strokeWidth='2'
            fill='none'
            strokeLinecap='round'
            strokeLinejoin='round'
            className='w-4'
          >
            <polyline points='9 18 15 12 9 6'></polyline>
          </svg>
        </a>
      )}
    </div>
  );
};

