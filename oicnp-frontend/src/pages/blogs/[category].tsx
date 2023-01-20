import React from 'react';
import Head from 'next/head';
import { SITE, mainMenu } from '../../constants';
import { QueryNodesResponseData } from '../../typings';
import { queryNodes } from '../../services';
import { Category } from '../../containers/Category';

export interface BlogsProps {
  nodesRes: QueryNodesResponseData;
  category: string;
}

const Blogs: React.FC<BlogsProps> = ({
  nodesRes,
  category,
}) => {

  return (
    <Category
      nodesRes={nodesRes}
      category={category}
    />
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

export const getStaticProps = async ({ params }: any) => {
  const category = params?.category;
  const { pageSize } = SITE;
  const nodesRes = await queryNodes({
    category,
    page: 1,
    pageSize,
  });

  return {
    props: {
      nodesRes,
      category,
    },
  };
};

export default Blogs;