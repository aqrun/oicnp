import { SITE } from '../../constants';
import { Home as HomeBase } from '../../containers';
import {
  QueryNodesResponseData,
} from '../../typings';
import { queryNodes } from '../../services';

export interface IndexPageProps {
  nodesRes: QueryNodesResponseData;
}

export const IndexPage: React.FC<IndexPageProps> = (props) => {

  return (<HomeBase {...props} />)
};

export const getStaticPaths1 = async () => {
  const { pageSize } = SITE;
  const nodesRes = await queryNodes({
    pageSize,
  });
  const totalCount = nodesRes?.totalCount;
  const totalPage = Math.ceil(totalCount / pageSize)

  let paths = [];
  for (let i = 2; i <= totalPage; i++) {
    paths.push(`/page/${i}`);
  }

  return { paths, fallback: false }
};

export const getStaticProps1 = async ({ params }: any) => {
  const page = Number(params.page || 1);
  const { pageSize } = SITE;
  const nodesRes = await queryNodes({
    page,
    pageSize,
  });

  return {
    props: {
      nodesRes,
    },
  };
};

export default IndexPage;