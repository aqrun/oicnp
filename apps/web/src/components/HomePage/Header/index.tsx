import { MAIN_MENUS } from '@/constant';

import { MenuToggleButton } from './MenuToggleButton';
import clsx from 'clsx';

import { HeaderMenu } from './index.styled';

export interface HeaderProps {
  activeMenuId?: string;
}

export const Header = ({
  activeMenuId = 'home',
}: HeaderProps) => {
  return (
    <section className='oic-header-section mb-6 overflow-hidden transition-colors duration-500 lg:z-50 shadow-md shadow-indigo-100/[0.6] dark:shadow-slate-100/[0.6] bg-white/95 supports-backdrop-blur:bg-white/60 dark:bg-transparent'>
      <div className='oic-header-inner h-auto w-screen'>
        {/* <!-- NAVBAR --> */}
        <nav className='font-inter mx-auto h-auto w-full max-w-[1600px] lg:relative lg:top-0'>
          {/* <!-- CONTAINER --> */}
          <div className='relative px-6 py-6 items-center lg:flex lg:items-center lg:justify-between lg:px-10 lg:py-4 xl:px-20'>
            <a
              href='/'
              aria-current='page'
              className='relative bg-[#00000000] no-underline hover:outline-0 max-[991px]:mr-auto max-[767px]:pl-0 text-xl lg:text-2xl font-bold text-gray-800 flex items-center hover:text-[var(--primary)]'
              aria-label='home'
            >
              {/* <img
                src='/favicon/logo112501.svg'
                loading='lazy'
                alt=''
                className='inline-block max-h-9 max-w-full'
              /> */}
              <span
                className='inline-block max-h-9 max-w-full h-11'
                dangerouslySetInnerHTML={{
                  __html: `<svg height="45" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" baseProfile="full" fill="currentColor">
    <g transform="scale(1,-1) translate(0,-500)">
        <path d="M 1.235852 486.524607 L 337.874486 486.524607 L 283.577932 374.311729 L 500.764148 1.475393 L 149.646433 1.475393 L 211.182527 110.068500 Z" transform="scale(1.000000,1.000000) translate(-1.000000,6.000000)" opacity="1.000000"></path>
        <path d="M 114.926122 211.338204 L 145.162852 157.004964 L 61.602733 1.475393 L 1.235852 1.475393 Z" transform="scale(1.000000,1.000000) translate(-1.000000,6.000000)" opacity="1.000000"></path>
        <path d="M 86.144257 210.524607 L 1.203135 55.144015 L 31.166850 1.913660 L 145.764148 210.524607 Z" transform="scale(1.000000,1.000000) translate(354.000000,282.000000)" opacity="1.000000"></path>
    </g>
</svg>`,
                }}
              />
            
              <span className="flex flex-col ml-1 min-w-24">
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
            <HeaderMenu className='oic-header-menu hidden lg:block'>
              <div className='mt-14 flex flex-col space-y-8 lg:mt-0 lg:flex lg:flex-row lg:space-x-1 lg:space-y-0'>
                {MAIN_MENUS?.map((item) => {
                  return (
                    <a
                      key={item?.vid}
                      href={item?.href}
                      className={clsx(`header-nav-item item-${item?.vid} font-inter text-center rounded-lg text-black lg:px-6 lg:py-4 lg:hover:bg-gray-50 lg:hover:text-gray-800`, {
                        'active': activeMenuId === item?.vid,
                      })}
                    >
                      {item?.name}
                    </a>
                  );
                })}
              </div>
            </HeaderMenu>
          </div>
        </nav>
      </div>
    </section>
  );
};
