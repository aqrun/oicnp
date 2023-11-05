import React from 'react';
import { GetServerSideProps } from 'next';
import { queryNode, queryNodes } from '../../../services';
import { Node } from '../../../typings';
import { Article } from '../../../containers/Article';

export interface BlogProps {
  node: Node,
  prevNode?: Node;
  nextNode?: Node;
}

const Blog: React.FC<BlogProps> = ({
  node,
  prevNode,
  nextNode,
}) => {
  return (
    <Article
      node={node}
      prevNode={prevNode}
      nextNode={nextNode}
    />
  );
}

export const getServerSideProps1: GetServerSideProps = async ({ params }) => {
  const slug = (params?.slug || '') as string;
  const nid = Number(params?.nid) || 0;

  const res = await queryNode({
    bundle: 'article',
    nid,
    vid: slug,
  });
  let node = res?.node;
  const relatedRes = await queryNodes({
    category: '', // node?.category?.name ?? '',
    page: 1,
    pageSize: 10,
    targetNid: node?.nid,
  });

  let prevNode = null;
  let nextNode = null;
  const relatedList = relatedRes?.nodes ?? [];

  if (relatedList.length) {
    const targetIndex = relatedList.findIndex(item => item.nid === node?.nid);

    if (targetIndex > 0) {
      prevNode = relatedList?.[targetIndex - 1] ?? null;
    }
    if (targetIndex < (relatedList.length - 1)) {
      nextNode = relatedList?.[targetIndex + 1] ?? null;
    }
  }

  return {
    props: {
      node,
      prevNode,
      nextNode,
    },
  };
};

export default Blog;
