import { bundleMDX } from 'mdx-bundler';
import { getMDXComponent, MDXContentProps } from 'mdx-bundler/client';
import React from 'react';
import rehypePrettyCode from 'rehype-pretty-code';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';
import remarkGfm from 'remark-gfm';
import remarkParse from 'remark-parse';

export const parseMdx = async (
  markdown: string
): Promise<React.FC<MDXContentProps>> => {
  const content = await bundleMDX({
    source: markdown,
    mdxOptions: (options) => {
      options.remarkPlugins = [
        ...(options?.remarkPlugins ?? []),
        remarkGfm,
        remarkParse,
        // remarkRehype,
      ];
      options.rehypePlugins = [
        ...(options?.rehypePlugins ?? []),
        rehypeSanitize,
        [
          rehypePrettyCode,
          {
            defaultLang: 'plaintext',
            theme: 'one-dark-pro',
          },
        ],
        rehypeStringify,
      ];
      return options;
    },
  });

  const Content = getMDXComponent(content?.code);

  return Content;
};
