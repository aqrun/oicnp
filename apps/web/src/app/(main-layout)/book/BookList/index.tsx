'use client';

import { useRouter } from 'next/navigation';
import { useMemo } from 'react';
import BookLayout from './BookLayout';
import { BookItem } from '@/content/books';
import { useBookStore } from '../useBookStore';
import { BookListContainer } from './index.styled';

export interface ArticleListProps {
  catVid?: string;
  tagVid?: string;
  books?: BookItem[];
}

export default function ArticleList({
  catVid,
  books,
}: ArticleListProps) {
  const router = useRouter();
  const category = useBookStore((state) => state.category);
  const title = useBookStore((state) => state.title);
  const setBookState = useBookStore.setState;

  const filteredBooks = useMemo(() => {
    let filteredBooks = books;

    if (category) {
      filteredBooks = filteredBooks?.filter((item) => {
        if (category === 'all') { return true;}
        return item?.category === category;
      });
    }

    return filteredBooks;
  }, [books, category]);

  const book = useMemo(() => {
    if (!title) return filteredBooks?.[0];
    return filteredBooks?.find((item) => {
      return item?.title === title || item?.chapter === title;
    });
  }, [filteredBooks, title]);

  return (
    <BookLayout
      catVid={catVid}
    >
      <BookListContainer className='relative flex flex-wrap flex-row'>
        {filteredBooks?.map((item) => {
          return (
            <div
              key={item?.title || item?.chapter}
              className="book-item"
            >
              <a
                className="item-inner"
                onClick={() => {
                  setBookState({ title: item?.title || item?.chapter });
                  router.push(`/book#/detail/${item?.title || item?.chapter}`);
                }}
              >
                <div className="book-item-title">
                  {item?.title || item?.chapter}
                </div>
                {item?.author && (
                  <div className="book-item-author">
                    （{item?.author}）
                  </div>
                )}
              </a>
            </div>
          );
        })}
      </BookListContainer>
      
      {book && (
        <div className="mt-8 min-h-[800px]  ">
          <h1 className="text-2xl font-bold mb-4">{book?.title || book?.chapter}</h1>
          <p className=" text-gray-500 mb-4">{book?.author}</p>
          {book?.content?.map((n, index) => {
            return (
              <p className={`${index} mb-4`}>
                {n}
              </p>
            );
          })}
        </div>
      )}
    </BookLayout>
  );
}