export interface BookItem {
  id?: number;
  uuid?: string;
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
  tags?: string[];
}