'use client';

import {useState } from 'react';
import BaseImage, { ImageProps as BaseImageProps } from 'next/image';
import { LOGO } from '@repo/utils/client';

export interface ImageProps extends BaseImageProps {
  
}

export function Image(props: ImageProps) {
  const [showFallback, setShowFallback] = useState(false);

  if (showFallback) {
    return (
      <span
        className='lx-image-container flex items-center justify-center max-h-full max-w-full'
        dangerouslySetInnerHTML={{
          __html: LOGO,
        }}
      />
    );
  }

  return (
    <BaseImage
      {...props}
      onError={() => {
        setShowFallback(true);
      }}
    />
  );
}