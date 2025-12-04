import BlogHero from './BlogHero';
import SideNav from './SideNav';
import {
  SideBar,
} from '@/components/HomePage';

export interface BlogLayoutProps extends React.PropsWithChildren {
  hasBlogHero?: boolean;
  hasSideNav?: boolean;
  catVid?: string;
  hasSideBar?: boolean;
}

export default function BlogLayout({
  children,
  hasBlogHero = true,
  hasSideNav = true,
  hasSideBar = true,
  catVid,
}: BlogLayoutProps): JSX.Element {
  return (
    <div className="layout">
      {hasBlogHero && <BlogHero />}
      <div className="flex gap-4 mb-8">
        {hasSideNav && <SideNav catVid={catVid} />}
        <div className='oic-layout-content flex flex-col'>
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