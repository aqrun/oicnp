import React from 'react';
import dynamic from 'next/dynamic';

const SocialShareBase = dynamic(
  () => import('./SocialShareBase'),
  { ssr: false, }
);

export interface SocialShareProps {

}

export const SocialShare: React.FC<SocialShareProps> = (props) => {

  return (
    <SocialShareBase {...props} />
  );
};
