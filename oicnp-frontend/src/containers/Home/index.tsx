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
}

export const Home: React.FC<HomeProps> = ({
  nodesRes,
}) => {
  return (
    <Container className="oic-Home-container">
      <HtmlHead />
      <Header menuId={MenuId.index} />

      {/* https://www.gatsbyjs.com/ */}
      <main className="mx-auto grid">
        <section className="oic-banner pb-20 pt-20 bg-white mt-6">
          <div className="mx-auto px-6 w-11/12 max-w-7xl">
            <header className="flex flex-row items-center gap-12">
              <div className="oic-header-text-w grid gap-6 justify-items-start w-1/2 text-black">
                <div className="grid gap-2 max-w-4xl">
                  <h1 className="text-4xl font-bold text-black-500">
                    <b className="text-purple">OICNP</b>
                    <br/>
                    Oh I See No Problem!
                  </h1>
                </div>
                <div className="max-w-2xl whitespace-pre-wrap leading-normal text-xl">
                  <p>
                    不仅是技术，更是梦想！再牛逼的梦想也抵不住傻逼似的坚持！
                    坚持一件事情，并不是因为这样做了会有效果，而是坚信，这样做是对的。
                  </p>
                </div>
                <div className="flex flex-row justify-center">
                  <a className="oic-btn-default min-w-[15rem]" >
                    一穷二白
                  </a>
                  <a className="oic-btn-simple ml-5 min-w-[15rem] hover:underline hover:underline-offset-4" >
                    充满希望
                  </a>
                </div>
              </div>
              <div className="oic-header-img-w w-1/2">
                <div className="oic-image-w max-w-[800px] relative">
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
          <div className="oic-home-main-inner mx-auto max-w-7xl flex flex-row">
            <div
              className="oic-home-body flex-1 mr-5 mb-5 bg-white px-5"
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
            
            <div className="oic-home-side w-96 relative">
              <BloodRecommend />
              <RandomRecommend />
              <HotTags />
              <SiteInfo />
            </div> {/* sidebar end */}

          </div>
        </section>

        <Projects />

        <section
          className="oic-home-roster bg-purple-900"
        >
          <div
            className="oic-inner py-20 mx-auto max-w-7xl flex flex-col items-center justify-center"
          >
            <h2 className="text-white font-bold text-4xl">
              不求大师的水平，只需分享、参与的热情！
            </h2>
            <div
              className=" text-gray-400 mt-6 mx-auto w-2/3 text-center text-lg"
            >
              几乎能为任何应用程序或需求自动地作出优化和定制；
              对极限的配置、性能的追求，顶尖的用户和开发者；
              一起交流，解决问题，交流技术， 提高普及率。
            </div>
            <div className="flex flex-row justify-center mt-6">
              <a className="oic-btn-default min-w-[15rem]" >
                参与建设
              </a>
              <a className="oic-btn-simple ml-5 min-w-[15rem]" >
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

