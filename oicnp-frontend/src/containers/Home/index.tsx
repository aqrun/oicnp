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
      {/* <div
        className={`g-banner home-banner index-page banner-theme-${SITE.themeColor} h-96`}
        data-theme={SITE.themeColor}
      >
        <h2>title</h2>
        <h3>
          content
        </h3>
      </div> */}

      {/* https://www.gatsbyjs.com/ */}
      <main className="mx-auto grid">
        <section className="oic-banner pb-20 pt-20 bg-white">
          <div className="mx-auto px-6 w-11/12 max-w-7xl">
            <header className="flex flex-row items-center gap-12">
              <div className="oic-header-text-w grid gap-6 justify-items-start w-1/2 text-black">
                <div className="grid gap-2 max-w-4xl">
                  <h1 className="text-4xl font-bold text-black-500">
                    <b className="text-purple">Gatsby</b>
                    <br/>
                    is joining Netlify!
                  </h1>
                </div>
                <div className="max-w-2xl whitespace-pre-wrap leading-normal text-xl">
                  <p>
                    Accelerating Gatsby’s growth and bringing composable architectures to the whole web.
                  </p>
                </div>
                <div className="flex flex-row justify-center">
                  <a
                    className="inline-flex items-center cursor-pointer
                      justify-center no-underline text-white font-semibold
                      text-lg min-w-12 min-h-12 py-2 px-5 bg-purple
                      border border-solid border-purple rounded-lg
                      hover:bg-purple-500 transition-all duration-200
                      "
                  >
                    Read the announcement
                  </a>
                  <a
                    className="ml-5 inline-flex items-center cursor-pointer
                      justify-center text-purple-300 font-semibold no-underline
                      text-lg min-w-12 min-h-12 py-2 px-5 bg-transparent
                      border border-solid border-transparent rounded-lg
                      transition-all duration-200
                      hover:text-purple
                    "
                  >
                    Join the Webinar
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
            
            <div className="oic-home-side w-3/12 relative">
              side-bar
            </div> {/* sidebar end */}

          </div>
        </section>
      </main>

      <main className="g-container home-content">
        <div
          className={'font-bold underline'}
        >
          test text
        </div>
      </main>
      <LayoutFooter />
    </Container>
  )
};

