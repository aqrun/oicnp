import { SITE } from '../../constants';
import {
  QueryNodesResponseData,
  MenuId,
} from '../../typings';
import {
  LayoutFooter,
  HtmlHead,
  ArticleList,
  SideBar,
  Header,
} from '../../components';
import {
  Container,
} from './index.styled';
import Image from 'next/image';
import { ArticleItem } from './ArticleItem';
import { Projects } from './Projects'
import {
  HotTags,
  BloodRecommend,
  RandomRecommend,
  SiteInfo,
} from '~/components/widgets';

export interface HomeProps {
  nodesRes: QueryNodesResponseData;
  isMobile?: boolean;
}

export const Home: React.FC<HomeProps> = ({
  nodesRes,
  isMobile,
}) => {
  return (
    <Container className="oic-Home-container">
      <HtmlHead />
      <Header menuId={MenuId.index} />

      {/* https://www.gatsbyjs.com/ */}
      <main className="mx-auto grid">
        <section className="oic-banner pb-20 pt-20 bg-white mt-6">
          <div className="mx-auto md:px-6 md:w-11/12 max-w-7xl">
            <header className="flex flex-col-reverse md:flex-row items-center gap-12">
              <div className="oic-header-text-w grid gap-6 justify-items-start w-10/12 md:w-1/2 text-black">
                <div className="grid gap-2 max-w-4xl">
                  <h1 className="text-3xl md:text-4xl font-bold text-black-500">
                    <b className="text-purple">了息</b>
                    <br/>
                    寥寥无几，生生不息
                  </h1>
                </div>
                <div className="md:max-w-2xl whitespace-pre-wrap leading-normal text-xl">
                  <p>
                    不仅是技术，更是梦想！再牛逼的梦想也抵不住傻逼似的坚持！
                    坚持一件事情，并不是因为这样做了会有效果，而是坚信，这样做是对的。
                  </p>
                </div>
                <div className="flex flex-row justify-center">
                  <a className="oic-btn-default md:min-w-[15rem]" >
                    前路漫漫
                  </a>
                  <a className="oic-btn-simple ml-5 md:min-w-[15rem] hover:underline hover:underline-offset-4" >
                    充满希望
                  </a>
                </div>
              </div>
              <div className="oic-header-img-w w-10/12 md:w-1/2">
                <div className="oic-image-w md:max-w-[800px] relative">
                  <Image
                    src="/assets/img/home-banner.avif"
                    alt="banner"
                    layout="responsive"
                    width="800px"
                    height="456px"
                    className="max-w-800px w-auto h-auto"
                  />
                </div>
              </div>
            </header>
          </div>
        </section>

        <section
          className="oic-home-main-w mt-6"
        >
          <div className="oic-home-main-inner md:mx-auto max-w-7xl flex flex-col md:flex-row">
            <div
              className="oic-home-body flex-1 md:mr-5 mb-5 bg-white px-5"
            >
              <div
                className="oic-article-list"
              >
                
                {[1,2,3,4,5,6,7].map((item) => {
                  return (
                    <ArticleItem key={item} />
                  );
                })}
  
              </div>
            </div>
            
            <div className="oic-home-side md:w-96 relative">
              <BloodRecommend />
              <RandomRecommend />
              <HotTags />
              <SiteInfo />
            </div>

          </div>
        </section>

        <Projects />

        <section
          className="oic-home-roster bg-purple-900"
        >
          <div
            className="oic-inner py-20 mx-6 md:mx-auto max-w-7xl flex flex-col items-center justify-center"
          >
            <h2 className="text-white font-bold text-2xl md:text-4xl text-center">
              不求大师的水平，只需分享、参与的热情！
            </h2>
            <div
              className=" text-gray-400 mt-6 mx-auto md:w-2/3 text-center text-lg"
            >
              几乎能为任何应用程序或需求自动地作出优化和定制；
              对极限的配置、性能的追求，顶尖的用户和开发者；
              一起交流，解决问题，交流技术， 提高普及率。
            </div>
            <div className="flex flex-row justify-center mt-6">
              <a className="oic-btn-default md:min-w-[15rem]" >
                参与建设
              </a>
              <a className="oic-btn-simple ml-5 md:min-w-[15rem]" >
                我能做些什么？
              </a>
            </div>
          </div>
        </section>
      </main>

      <LayoutFooter />
    </Container>
  )
};

