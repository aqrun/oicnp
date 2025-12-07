import SideNav from './SideNav';
import {
  SideBar,
} from '@/components/HomePage';
import { HeroContainer } from './index.styled';

export interface BookLayout extends React.PropsWithChildren {
  hasBlogHero?: boolean;
  hasSideNav?: boolean;
  catVid?: string;
  hasSideBar?: boolean;
}

export default function BookLayout({
  children,
  hasBlogHero = true,
  hasSideNav = true,
  hasSideBar = true,
  catVid,
}: BookLayout): JSX.Element {
  return (
    <div className="layout">
      {hasBlogHero && (
        <HeroContainer
          id="blog-hero"
          className="flex flex-col items-center justify-center bg-center bg-cover bg-no-repeat py-0 px-1 text-white rounded-md mb-6"
        >
          <h1 className="blog-hero-title">内容阅读</h1>
          <div className="blog-hero-description">
            在线内容阅读，包括但不限于诗词、技术文档等。
          </div>
        </HeroContainer>
      )}
      <div className="flex lg:flex-row flex-col gap-4 mb-8">
        {hasSideNav && <SideNav catVid={catVid} />}
        <div className='oic-layout-content1 flex flex-col flex-1'>
          {children}
        </div>
        {hasSideBar && (
          <div className='lg:w-80'>
            <SideBar
              hasTags
            />
          </div>
        )}
      </div>
    </div>
  );
}