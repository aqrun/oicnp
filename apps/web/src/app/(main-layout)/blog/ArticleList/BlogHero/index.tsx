import {
  BlogHeroContainer
} from './index.styled';

export interface BlogHeroProps {}

export default function BlogHero({}: BlogHeroProps): JSX.Element {
  return (
    <BlogHeroContainer
      id="blog-hero"
      className="flex flex-col items-center justify-center bg-center bg-cover bg-no-repeat py-0 px-1 text-white rounded-md mb-6"
    >
      <h1 className="blog-hero-title">IT技术</h1>
      <div className="blog-hero-description">
        专注IT技术分享，包括但不限于前端、后端、数据库、操作系统、网络、安全等。
      </div>
    </BlogHeroContainer>
  );
}