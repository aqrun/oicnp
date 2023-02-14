import React from 'react';
import { SITE } from '../../constants';

export interface LayoutFooterProps {

}

export const LayoutFooter: React.FC<LayoutFooterProps> = () => {

  return (
    <footer className="g-footer">
      <section>
        { SITE.title } © 2014 - {(new Date).getFullYear()}
      </section>
      <section>
        Powered by&nbsp;
        <a
          href="https://nextjs.org/"
          target="_blank"
          rel="noreferrer"
          className="hover:text-purple-300"
        >Next.js</a> 
        <span className="mx-1">|</span>
        {/* <a
          href="https://tailwindcss.com/"
          target="_blank" rel="noreferrer"
          className="hover:text-purple-300"
        >Tailwind CSS</a> 
        <span className="mx-1">|</span> */}
        <a
          href="https://www.rust-lang.org/" target="_blank" rel="noreferrer"
          className="hover:text-purple-300"
        >Rust</a>
        {/* <span className="mx-1">|</span>
        <a
          href="https://github.com/poem-web/poem" target="_blank" rel="noreferrer"
          className="hover:text-purple-300"
        >poem</a>
        <span className="mx-1">|</span>
        <a
          href="https://github.com/async-graphql/async-graphql" target="_blank" rel="noreferrer"
          className="hover:text-purple-300"
        >async-graphql</a> */}
      </section>
    </footer>
  );
}
