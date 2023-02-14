import { SITE } from '../constants';
import { QueryNodesResponseData } from '../typings';
import { Home as HomeBase } from '../containers';
import { queryNodes } from '../services';
import { GetServerSideProps } from 'next';
import { checkIsMobile } from '~/utils';

export interface HomeProps {
  nodesRes: QueryNodesResponseData;
}

const Home: React.FC<HomeProps> = (props) => {
  return (<HomeBase {...props} />);
}

export const getServerSideProps: GetServerSideProps = async (ctx) => {
  const isMobile = checkIsMobile(ctx);
  const { pageSize } = SITE;
  const nodesRes = await queryNodes({
    page: 1,
    pageSize,
  });

  return {
    props: {
      nodesRes,
      isMobile,
    },
  };
};

export default Home;