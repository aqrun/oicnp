
export interface ToolItem {
  name: string;
  url?: string;
  description?: string;
  logo?: string;
  language?: string;
  /**
   * 工具分类 react-ui static-site-generator
   */
  category?: string;
}

export interface ToolCategories {
  id: string;
  name: string;
}