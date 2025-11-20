export interface BookItem {
  title?: string;
  section?: string;
  author?: string;
  chapter?: string;
  content?: string[];
  tags?: string;
  prologue?: string;
  category?: string;
}

export interface BookCategories {
  id: string;
  name: string;
}