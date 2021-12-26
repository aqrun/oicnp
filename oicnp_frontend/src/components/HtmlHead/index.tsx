import Head from 'next/head';
import { SITE } from '../../constants';

export const HtmlHead = () => {

  return (
    <Head>
      <meta charSet="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>{SITE.title}</title>
      <meta name="author"  content="子十" />
      <meta name="description" content={SITE.description} />
      <meta name="keywords"  content={SITE.keyword} />

      <meta property="og:title" content={SITE.title} />
      <meta property="og:type" content="website" />
      <meta property="og:url" content="http://www.aqrun.com" />
      <meta property="og:description" content={SITE.description} />
      <meta property="og:site_name" content={SITE.title} />

      <link rel="stylesheet" href="//cdn.staticfile.org/normalize/6.0.0/normalize.min.css" />
      <link rel="stylesheet" href="//at.alicdn.com/t/font_3063613_38mhesfhyb5.css"/>
      <link rel="stylesheet" href="/assets/css/github-markdown.css" />
      <link rel="stylesheet" href="/assets/prism/prism.css?v=0.1" />
      <link rel="stylesheet" href="/assets/css/share.min.css" />
      <link rel="stylesheet" href="https://cdn.bootcss.com/font-awesome/4.7.0/css/font-awesome.min.css" />
    </Head>
  );
};
