'use client';

export interface SideBarBlockProps {
  title: string;
  children: React.ReactNode;
}

export function SideBarBlock({
  title,
  children,
}: SideBarBlockProps) {
  return (
    <div className='pb-4 card-base border border-slate-200 mt-4 rounded-lg hover:shadow-md'>
      <div className='transition before:absolute before:bg-[var(--primary)] before:rounded-md before:w-1 font-bold before:h-4 dark:text-neutral-100 relative text-lg text-neutral-900 before:left-[-16px] before:top-[8.5px] mb-2 ml-8 mt-4'>
        {title}
      </div>
      <div className='overflow-hidden collapse-wrapper px-4'>
        {children}
      </div>
    </div>
  );
}