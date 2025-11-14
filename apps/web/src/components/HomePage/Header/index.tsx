import { MAIN_MENUS } from '@/constant';

import { MenuToggleButton } from './MenuToggleButton';

export const Header = () => {
  return (
    <section className='oic-header-section mb-6 overflow-hidden transition-colors duration-500 lg:z-50 shadow-md shadow-indigo-100/[0.6] dark:shadow-slate-100/[0.6] bg-white/95 supports-backdrop-blur:bg-white/60 dark:bg-transparent'>
      <div className='h-auto w-screen'>
        {/* <!-- NAVBAR --> */}
        <nav className='font-inter mx-auto h-auto w-full max-w-[1600px] lg:relative lg:top-0'>
          {/* <!-- CONTAINER --> */}
          <div className='relative px-6 py-6 items-center lg:flex lg:items-center lg:justify-between lg:px-10 lg:py-4 xl:px-20'>
            {/* <!-- SVG LOGO - YOU CAN REPLACE THIS --> */}
            <a
              href='/'
              aria-current='page'
              className='relative bg-[#00000000] no-underline hover:outline-0 max-[991px]:mr-auto max-[767px]:pl-0 text-2xl lg:text-3xl font-bold text-gray-800 flex items-center hover:text-[var(--primary)]'
              aria-label='home'
            >
              <img
                src='/favicon/logo.png'
                loading='lazy'
                alt=''
                className='inline-block max-h-8 max-w-full'
              />
              <span className="flex flex-col ml-1">
                <span className="">
                  灵犀纪
                </span>
                {/* <span className='text-[12px] text-gray-400 font-normal'>
                  LXAGE.COM
                </span> */}
              </span>
            </a>
            <div className='absolute right-4 top-7 lg:hidden'>
              <MenuToggleButton />
            </div>
            {/* <!-- MENU CONTENT 1 --> */}
            <div className='oic-header-menu hidden lg:block'>
              <div className='mt-14 flex flex-col space-y-8 lg:mt-0 lg:flex lg:flex-row lg:space-x-1 lg:space-y-0'>
                {MAIN_MENUS?.map((item) => {
                  return (
                    <a
                      key={item?.vid}
                      href={item?.href}
                      className='font-inter text-center rounded-lg text-black lg:px-6 lg:py-4 lg:hover:bg-gray-50 lg:hover:text-gray-800'
                    >
                      {item?.name}
                    </a>
                  );
                })}
              </div>
            </div>
          </div>
        </nav>
      </div>
    </section>
  );
};
