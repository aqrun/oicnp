import BookLayout from '../BookList/BookLayout';
import { BookItem } from '@/content/books';
import { PoemItem } from './PoemItem';
import { BookHomeContainer } from './index.styled';

export interface BookHomeProps {
  catVid?: string;
  tagVid?: string;
  books?: BookItem[];
}

export default function BookHome({
  catVid,
  books,
}: BookHomeProps) {

  return (
    <BookLayout
      catVid={catVid}
    >
      <BookHomeContainer className='relative flex flex-wrap flex-row gap-3'>
        {books?.slice(0, 10)?.map((item) => {
          return (
            <PoemItem key={item?.title} record={item} />
          );
        })}
      </BookHomeContainer>
    </BookLayout>
  );
}
