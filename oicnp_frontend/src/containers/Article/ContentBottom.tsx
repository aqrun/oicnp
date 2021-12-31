import React from 'react';
import { SITE } from '../../constants';
import {
  QueryNodeResponseData,
} from '../../typings';
import Link from 'next/link';
import { PagerItem } from './PagerItem';

export interface ContentBottomProps {
  prevNode?: QueryNodeResponseData['node'];
  nextNode?: QueryNodeResponseData['node'];
}

export const ContentBottom: React.FC<ContentBottomProps> = ({
  prevNode,
  nextNode,
}) => {

  return (
    <section className="author-detail">
      <section className="post-footer-item author-card">
        <div className="avatar">
          <img src={SITE.avatar} alt="" />
        </div>
        <div className="author-name">{SITE.author}</div>
        <div className="bio">
          <p dangerouslySetInnerHTML={{ __html: SITE.bio }} />
        </div>
        {SITE.sns.length && (
          <ul className="sns-links">
            {SITE.sns.map((snsItem) => {
              return (
                <li key={snsItem.name}>
                  <a href={snsItem.url} target="_blank" rel="noreferrer">
                    <i className={`iconfont icon-${snsItem.name}`}></i>
                  </a>
                </li>
              );
            })}
          </ul>
        )}
      </section>

      {(prevNode || nextNode) && (
        <section className="post-footer-item read-next">
          {prevNode && <PagerItem node={prevNode} className="oic-prev" />}
          {nextNode && <PagerItem node={nextNode} className="oic-next" />}
        </section>
      )}
    </section>
  );
};
