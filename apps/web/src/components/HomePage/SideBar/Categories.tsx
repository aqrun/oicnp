import { getTaxonomiesCount } from '@/utils';

export const Categories = () => {
  const taxonomies = getTaxonomiesCount();
  return (
    <div className='pb-4 card-base border border-slate-200 mt-4 rounded-lg hover:shadow-md'>
      <div className='transition before:absolute before:bg-[var(--primary)] before:rounded-md before:w-1 font-bold before:h-4 dark:text-neutral-100 relative text-lg text-neutral-900 before:left-[-16px] before:top-[5.5px] mb-2 ml-8 mt-4'>
        分类
      </div>
      <div className='overflow-hidden collapse-wrapper px-4'>
        {taxonomies?.categories?.map((item) => {
          return (
            <a href={item?.href} key={item?.vid}>
              <button className='dark:hover:text-[var(--primary)] hover:text-[var(--primary)] active:bg-[var(--btn-plain-bg-active)] bg-none hover:bg-[var(--btn-plain-bg-hover)] rounded-lg dark:text-neutral-300 h-10 hover:pl-3 pl-2 text-neutral-700 transition-all w-full'>
                <div className='flex items-center justify-between mr-2 relative'>
                  <div className='overflow-hidden overflow-ellipsis text-left whitespace-nowrap'>
                    {item?.name}
                  </div>
                  <div className='transition rounded-lg flex items-center text-[var(--btn-content)] text-sm bg-[oklch(0.95_0.025_var(--hue))] dark:bg-[var(--primary)] dark:text-[var(--deep-text)] font-bold h-7 justify-center min-w-[2rem] ml-4'>
                    {item?.count}
                  </div>
                </div>
              </button>
            </a>
          );
        })}
      </div>
    </div>
  );
};
