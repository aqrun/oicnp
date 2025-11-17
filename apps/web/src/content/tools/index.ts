import { AI_APPS_ITEMS } from './ai-apps';
import { CHARTS_ITEMS } from './charts';
import { FORMS_ITEMS } from './forms';
import { HEADLESS_CMS_ITEMS } from './headless-cms';
import { MARKDOWN_MDX_ITEMS } from './markdown-mdx';
import { REACT_UI_ITEMS } from './react-ui';
import { STATIC_SITES } from './static-sites';

export * from './base';
export * from './types';

export const ALL_TOOLS = [
  ...AI_APPS_ITEMS,
  ...CHARTS_ITEMS,
  ...FORMS_ITEMS,
  ...HEADLESS_CMS_ITEMS,
  ...MARKDOWN_MDX_ITEMS,
  ...REACT_UI_ITEMS,
  ...STATIC_SITES,
];