'use client';

import Image from 'next/image';
import { PhotoProvider,PhotoView } from 'react-photo-view';

export const Aotai = () => {
  return (
    <>
      <div className="mt-4">
        <p className=" mb-2 text-gray-500">
          鳌太路线手绘示意图, 仅供参考。
        </p>
        <div>
          <PhotoProvider>
            <PhotoView
              src="https://cdn.oicnp.com/images/2024/aotai.jpg"
            >
              <Image
                src='https://cdn.oicnp.com/images/2024/aotai.jpg'
                alt=''
                className='inline-block h-full w-full object-cover'
                width={1280}
                height={960}
              />
            </PhotoView>
          </PhotoProvider>
        </div>
      </div>

      <div className="mt-4">
        <p className=" mb-2 text-gray-500">
          秦岭经典路线
        </p>
        <div>
          <PhotoProvider>
            <PhotoView
              src="https://cdn.oicnp.com/images/2024/classic_roads.jpg"
            >
              <Image
                src='https://cdn.oicnp.com/images/2024/classic_roads.jpg'
                alt=''
                className='inline-block h-full w-full object-cover'
                width={1280}
                height={1708}
              />
            </PhotoView>
          </PhotoProvider>
        </div>
      </div>

      <div className="mt-4">
        <p className=" mb-2 text-gray-500">
          西安户外爬山鄙视链
        </p>
        <div>
          <PhotoProvider>
            <PhotoView
              src="https://cdn.oicnp.com/images/2024/qinlinglian.jpg"
            >
              <Image
                src='https://cdn.oicnp.com/images/2024/qinlinglian.jpg'
                alt=''
                className='inline-block h-full w-full object-cover'
                width={1170}
                height={1556}
              />
            </PhotoView>
          </PhotoProvider>
        </div>
      </div>
    </>
  );
};

