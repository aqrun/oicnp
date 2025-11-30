'use client';

import styled from 'styled-components';

export const HeroContainer = styled.div`
  background-image: url('/images/heros/blog.png');
  height: 200px;
`;

export const BookHomeContainer = styled.div`
  .book-item {
    margin-bottom: 8px;
    font-size: 12px;
    
    .item-inner {
      display: flex;
      align-items: center;
      cursor: pointer;
      width: 200px;

      &:hover {
        color: var(--primary);
      }
    }
  }
`;