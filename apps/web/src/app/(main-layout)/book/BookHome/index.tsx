import BookLayout from '../BookList/BookLayout';
import { PoemItem } from './PoemItem';
import { ListBlockTitle } from '@/components';
import { PoetryListPageDataModel, ChapterModel } from '@repo/apis/server';
import { BOOK_CATEGORIES } from '@/content/books/base';
import { BookHomeContainer } from './index.styled';

export interface BookHomeProps {
  catVid?: string;
  tagVid?: string;
  books?: PoetryListPageDataModel[];
  chapters?: ChapterModel[];
}

export default function BookHome({
  catVid,
  books,
  chapters,
}: BookHomeProps) {
  return (
    <BookLayout
      catVid={catVid}
    >
      {BOOK_CATEGORIES?.filter((item) => item?.id !== 'all')?.map((category) => {
        const poetry = books?.filter((book) => {
          const exist = category?.tags?.some((tag) => {
            return book?.tags?.indexOf(tag) !== -1;
          });
          return exist;
        })?.slice(0, 6);

        return (
          <div key={category?.id}>
            <ListBlockTitle
              title={category?.name}
              moreLink="/book/c/chu_ci"
            />
            <BookHomeContainer className='relative flex flex-wrap flex-row'>
              {poetry?.map((item) => {
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
            </BookHomeContainer>
          </div>
        );
      })}
    </BookLayout>
  );
}
