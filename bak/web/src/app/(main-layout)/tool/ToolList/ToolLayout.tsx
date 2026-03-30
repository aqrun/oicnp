import SideNav from './SideNav';
import { ToolCategories } from '@/content/tools';
import { HeroContainer } from './index.styled';

export interface BlogLayoutProps extends React.PropsWithChildren {
  hasHero?: boolean;
  hasSideNav?: boolean;
  catVid?: string;
  hasSideBar?: boolean;
  toolCategories?: ToolCategories[];
}

export default function ToolLayout({
  children,
  hasHero = true,
  hasSideNav = true,
  catVid,
  toolCategories,
}: BlogLayoutProps): JSX.Element {
  return (
    <div className="layout">
      {hasHero && (
        <HeroContainer
          id="blog-hero"
          className="flex flex-col items-center justify-center bg-center bg-cover bg-no-repeat py-0 px-1 text-white rounded-md mb-6"
        >
          <h1 className="blog-hero-title">常用工具</h1>
          <div className="blog-hero-description px-4">
            常用工具列表，包括但不限于AI助手、代码编辑器、图像生成器、翻译工具、PDF阅读器等。
          </div>
        </HeroContainer>
      )}
      <div className="flex lg:flex-row flex-col gap-4 mb-8">
        {hasSideNav && <SideNav catVid={catVid} toolCategories={toolCategories} />}
        <div className='oic-layout-content1 flex flex-col'>
          {children}
        </div>
      </div>
    </div>
  );
}