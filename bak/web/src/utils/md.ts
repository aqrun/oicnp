import rehypeStringify from 'rehype-stringify'
import remarkParse from 'remark-parse'
import remarkRehype from 'remark-rehype'
import {unified} from 'unified'
import remarkGfm from 'remark-gfm';
import rehypeSanitize from 'rehype-sanitize'
import rehypePrettyCode from 'rehype-pretty-code';

export async function parseMd(markdown: string) {
  const file = await unified()
    .use(remarkGfm)
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeSanitize)
    .use(rehypePrettyCode, {
      defaultLang: {
        block: 'plaintext',
        inline: 'plaintext',
      },
      theme: 'github-light',
      keepBackground: false,
    })
    .use(rehypeStringify)
    .process(markdown);

  return String(file);
}
