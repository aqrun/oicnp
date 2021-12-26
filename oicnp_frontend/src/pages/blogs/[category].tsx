import Head from 'next/head';
import { mainMenu } from '../../constants';

const Blogs = () => {

  return (
    <>
      <Head>
        <title>About 23</title>
      </Head>
      <div>
        about page
      </div>
    </>
  );
};

export const getStaticPaths = async () => {
  const paths = mainMenu.filter((item) => {
    return item.href !== '/';
  }).map((item) => {
    return item.href;
  });

  return { paths, fallback: false }
};

export const getStaticProps = async ({ params }) => {
  console.log('----', params);
  return {
    props: {

    },
  };
};

export default Blogs;