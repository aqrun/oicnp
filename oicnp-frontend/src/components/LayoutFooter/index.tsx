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
        Powered by <a href="//jekyllrb.com">Next.js</a> 
        | <a href="https://github.com/kaeyleo/jekyll-theme-H2O">Theme H2O</a>
      </section>
    </footer>
  );
}
