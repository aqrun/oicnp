import BookLayout from './BookLayout';
import { PoetryListPageDataModel, ChapterModel } from '@repo/apis/server';
import { PoemItem } from '../BookHome/PoemItem';
import { BOOK_CATEGORIES } from '@/content/books/base';
import { LoadMore } from './LoadMore';
import { BookListContainer } from './index.styled';

export interface ArticleListProps {
  catVid?: string;
  tagVid?: string;
  books?: PoetryListPageDataModel[];
  chapters?: ChapterModel[];
  needLoadMore?: boolean;
}

export default function BookList({
  catVid,
  books,
  chapters,
  needLoadMore,
}: ArticleListProps) {
  const category = BOOK_CATEGORIES.find((item) => item?.id === catVid);

  return (
    <BookLayout
      catVid={catVid}
    >
      <BookListContainer className='relative flex flex-wrap flex-row'>
        {books?.map((item) => {
          const bookChapters = chapters?.filter((chapter) => {
            return chapter?.poetryId === item?.id;
          });

          return (
            <PoemItem
              key={item?.id}
              category={category?.name}
              record={item}
              chapters={bookChapters}
            />
          );
        })}

        <LoadMore
          catId={catVid}
          needLoadMore={needLoadMore}
        />
      </BookListContainer>
    </BookLayout>
  );
}