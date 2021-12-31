import React, { useEffect, useMemo } from 'react';
import { SITE } from '../../constants';
import {
  Node,
  MenuId,
} from '../../typings';
import Link from 'next/link';
import {
  LayoutFooter,
  HtmlHead,
  ArticleList,
  SideBar,
  Header,
} from '../../components';
import { ArticleBody } from './ArticleBody';
import { Container } from './index.styled';
import { ContentBottom } from './ContentBottom';

export interface ArticleProps {
  node: Node;
  prevNode?: Node;
  nextNode?: Node;
}

export const Article: React.FC<ArticleProps> = ({
  node,
  prevNode,
  nextNode,
}) => {
  const headerCls = useMemo(() => {
    return `g-banner post-header
      post-pattern-${SITE.postPatterns} 
      bgcolor-${SITE.themeColor} 
      ${node ? '' : 'post-no-cover' }
    `;
  }, [node]);

  return (
    <Container>
      <HtmlHead />
      <Header
        menuId={MenuId.index}
      />

      <header
        className={headerCls}
        data-theme={SITE.themeColor}
      >
        <div className="post-wrapper">
          <div className="post-tags">
            {node?.tags?.map(item => {
              return (
                <span
                  key={item?.vid}
                  className="post-tag"
                >
                  <Link
                    href=""
                  >
                    {item.name}
                  </Link>
                </span>
              );
            })}
          </div>
          <h1>{node?.title}</h1>
          <div className="post-meta">
            <span
              className="post-meta-item"
            >
              <i className="iconfont icon-author" />
              {node?.author?.nickname || SITE.author}
            </span>
            <time
              className="post-meta-item"
              data-datetime={node?.createdAt}
            >
              <i className="iconfont icon-date" />
              {node?.createdAt}
            </time>
          </div>
        </div>
        {node?.cover && (
          <>
            <div className="filter" />
            <div
              className="post-cover"
              style={{
                background: `url('${node?.cover}') center no-repeat`,
                backgroundSize: 'cover',
              }}
            />
          </>
        )}
      </header>

      <div className="post-content visible">
        {node?.subTitle && (
          <h2
            className="post-subtitle"
          >
            {node?.subTitle}
          </h2>
        )}

        <article className="markdown-body">
          <ArticleBody
            body={node?.nodeBody?.body}
            bodyFormat={node?.nodeBody?.bodyFormat}
          />
        </article>

        {SITE?.socialShare && (
          <div className="social-share-wrapper">
            <div className="social-share"></div>
          </div>
        )}
      </div>

      <ContentBottom
        prevNode={prevNode}
        nextNode={nextNode}
      />

      <LayoutFooter />
    </Container>
  )
};