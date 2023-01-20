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
  Global,
} from './index.styled';

export interface HomeProps {
  nodesRes: QueryNodesResponseData;
}

export const Home: React.FC<HomeProps> = ({
  nodesRes,
}) => {
  return (
    <>
      <Global />
      <HtmlHead />
      <Header menuId={MenuId.index} />
      <div
        className={`g-banner home-banner index-page banner-theme-${SITE.themeColor}`}
        data-theme={SITE.themeColor}
      >
        <h2>满江红·怒发冲冠</h2>
        <h3>怒发冲冠，凭阑处、潇潇雨歇。抬望眼，仰天长啸，壮怀激烈。三十功名尘与土，八千里路云和月。莫等闲、白了少年头，空悲切。<br/>靖康耻，犹未雪。臣子恨，何时灭。驾长车，踏破贺兰山缺。壮志饥餐胡虏肉，笑谈渴饮匈奴血。待从头、收拾旧山河，朝天阙。</h3>
      </div>

      <main className="g-container home-content">
        <ArticleList
          allBlogs={nodesRes?.nodes ?? []}
          page={nodesRes?.page}
          pageSize={nodesRes?.pageSize}
          totalCount={nodesRes?.totalCount}
        />
        <SideBar />
      </main>
      <LayoutFooter />
    </>
  )
};
