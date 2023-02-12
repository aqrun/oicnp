import React from 'react';
import { SITE } from '../../constants';
import {
  Container
} from './index.styled';

export interface LayoutFooterProps {

}

export const LayoutFooter: React.FC<LayoutFooterProps> = () => {

  return (
    <footer className="g-footer">
      <section>
        { SITE.title } © 2014 - {(new Date).getFullYear()}
      </section>
      <section>
        Powered by <a href="https://nextjs.org/" target="_blank" rel="noreferrer">Next.js</a> 
        &nbsp;|&nbsp;
        <a href="https://www.rust-lang.org/" target="_blank" rel="noreferrer">Rust</a>
      </section>
    </footer>
  );
}
