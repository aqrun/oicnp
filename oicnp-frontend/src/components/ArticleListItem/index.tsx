import React from 'react';
import {
  Article
} from './index.styled';

export interface ArticleListItemProps {
  /**
   * 大图
   */
  cover?: string;
  /**
   * 文章链接
   */
  url: string;
  /**
   * 文章标题
   */
  title: string;
  /**
   * 子标题
   */
  subTitle?: string;
  excerpt?: string;
  tags?: string[];
  date?: string
}

export const ArticleListItem: React.FC<ArticleListItemProps> = ({
  cover,
  url,
  title,
  subTitle,
  excerpt,
  tags = [],
  date,
}) => {
  return (
    <Article className="article-item">
      {cover && (
        <div className="post-cover">
          <a
            className="post-link"
            href={url}
            title={title}
          ></a>
          <img
            src={cover}
            alt={title}
          />
        </div>
      )}
      <section className="post-preview">
          <a
            className="post-link"
            href={url}
            title={title}
          ></a>
          <h2 className="post-title">{title}</h2>
          {subTitle && (
            <h3 className="post-subtitle">{subTitle}</h3>
          )}
          {(!subTitle) && (
            <p
              className="post-excerpt"
              dangerouslySetInnerHTML={{
                __html: excerpt || '',
              }}
            />
          )}
      </section>
      <footer className="post-meta">
        <div className="post-tags">
          {tags.map((item) => {
            return (<a
              key={item}
              href=""
              className="post-tag"
            >{item}</a>);
          })}
        </div>
        <time
          className="post-date"
          dateTime="%y-%m-%d"
        >{date}</time>
      </footer>
    </Article>
  );
}