import React from 'react';

export interface ListBlockTitleProps {
  title: string;
  moreLink?: string;
}

export default function ListBlockTitle({
  title,
  moreLink,
}: ListBlockTitleProps): JSX.Element {
  return (
    <div className="list-block-title flex items-end justify-between mb-4 border-b border-gray-200 pb-1">
      <div className='flex items-center'>
        <h2 className="list-block-title-text text-xl font-medium">
          {title}
        </h2>
      </div>
      <div>
        {Boolean(moreLink) && (
          <a href={moreLink} className='text-sm text-gray-400 hover:text-primary'>
            更多
          </a>
        )}
      </div>
    </div>
  );
}