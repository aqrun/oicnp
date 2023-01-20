import React, { useEffect, useMemo, useState } from 'react';
import { unified } from 'unified';
import stream from 'unified-stream';
import remarkParse from 'remark-parse';
import remarkToc from 'remark-toc';
import remarkRehype from 'remark-rehype';
import remarkGfm from 'remark-gfm';
import rehypeDocument from 'rehype-document';
import rehypeFormat from 'rehype-format';
import rehypeStringify from 'rehype-stringify';
import { Content as ContentDiv } from './index.styled';
import { useMemoizedFn } from 'ahooks';
import rehypeHighlight from 'rehype-highlight'

export interface ArticleBodyProps {
  body: string;
  bodyFormat?: string;
}

export const ArticleBody: React.FC<ArticleBodyProps> = ({
  body,
  bodyFormat,
}) => {
  const [text, setText] = useState('');
  const generateContent = useMemoizedFn(() => {
    if (bodyFormat === 'markdown') {
      unified()
        .use(remarkParse)
        .use(remarkToc)
        .use(remarkRehype)
        .use(rehypeHighlight)
        .use(remarkGfm)
        // .use(rehypeDocument, {title: 'Contents'})
        .use(rehypeFormat)
        .use(rehypeStringify)
        .process(body)
        .then((con) => {
          setText(String(con));
        });
    } else {
      setText(body);
    }
  });

  const html = useMemo(() => ({ __html: text }), [text]);

  useEffect(() => {
    generateContent();
  }, []);

  return (
    <ContentDiv
      className="oic-content-container"
      dangerouslySetInnerHTML={html}
    >
    </ContentDiv>
  );
}
