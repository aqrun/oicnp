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
            专注IT技术分享，包括但不限于前端、后端、数据库、操作系统、网络、安全等。
          </div>
        </HeroContainer>
      )}
      <div className="flex gap-4 mb-8">
        {hasSideNav && <SideNav catVid={catVid} />}
        <div className='oic-layout-content1 flex flex-col'>
          {children}
        </div>
        {hasSideBar && (
          <div className='lg:w-80'>
            <SideBar
              hasWeather
              hasTags
            />
          </div>
        )}
      </div>
    </div>
  );
}