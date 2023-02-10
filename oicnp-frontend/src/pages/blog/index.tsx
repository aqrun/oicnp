import { SITE } from '~/constants';
import { QueryNodesResponseData } from '~/typings';
import { Home as HomeBase } from '~/containers';
import { queryNodes } from '~/services';

export interface HomeProps {
  nodesRes: QueryNodesResponseData;
}

const Home: React.FC<HomeProps> = (props) => {
  return (<HomeBase {...props} />);
}

export const getStaticProps = async () => {
  const { pageSize } = SITE;
  const nodesRes = await queryNodes({
    page: 1,
    pageSize,
  });

  return {
    props: {
      nodesRes,
    }
  }
};

export default Home;
