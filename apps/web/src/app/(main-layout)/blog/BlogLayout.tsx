import BlogHero from './BlogHero';
import SideNav from './SideNav';

export interface BlogLayoutProps extends React.PropsWithChildren {
  hasBlogHero?: boolean;
  hasSideNav?: boolean;
  catVid?: string;
}

export default function BlogLayout({
  children,
  hasBlogHero = true,
  hasSideNav = true,
  catVid,
}: BlogLayoutProps): JSX.Element {
  return (
    <div className="layout">
      {hasBlogHero && <BlogHero />}
      <div className="flex gap-4 mb-8">
        {hasSideNav && <SideNav catVid={catVid} />}
        {children}
      </div>
    </div>
  );
}