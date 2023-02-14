import React, { useMemo } from 'react';
import Head from 'next/head';
import { SITE } from '../../constants';

export interface HtmlHeadProps {
  url?: string;
  title?: string;
  description?: string;
  keywords?: string;
  author?: string;
}

export const HtmlHead: React.FC<HtmlHeadProps> = ({
  url,
  title,
  description,
  keywords,
  author,
}) => {
  const newTitle = title || SITE.title;
  const newUrl = url || SITE.url;
  const newDesc = description || SITE.description;
  const newKeywords = keywords || SITE.keyword;
  const newAuthor = author || SITE.author;

  return (
    <Head>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width" />
      <title>{newTitle}</title>
      <meta name="title" content={newTitle} />
      <meta name="author"  content={newAuthor} />
      <meta name="description" content={newDesc} />
      <meta name="keywords"  content={newKeywords} />

      <meta property="og:title" content={newTitle} />
      <meta property="og:type" content="website" />
      <meta property="og:url" content={newUrl} />
      <meta property="og:description" content={newDesc} />
      <meta property="og:site_name" content={SITE.title} />
    </Head>
  );
};
