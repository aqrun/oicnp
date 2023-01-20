import React, { useEffect, useState } from 'react';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

const minScrollTop = 100;

export const FixedButtons: React.FC = () => {
  const [show, setShow] = useState(false);

  const winScrollHandle = useMemoizedFn(() => {
    if (document.body.scrollTop > minScrollTop || document.documentElement.scrollTop > minScrollTop) {
      setShow(true);
    } else {
      setShow(false);
    }
  });

  const backTopHandle = useMemoizedFn(() => {
    document.body.scrollTop = 0;
    document.documentElement.scrollTop = 0;
  });

  useEffect(() => {
    winScrollHandle();
    window.onscroll = winScrollHandle;

    return () => {
      window.onscroll = null;
    };
  }, []);

  return (
    <Container className={`oic-fixed-btns ${show ? 'show' : ''}`}>
      <div className="oic-btn oic-back-top" onClick={backTopHandle}>
        <i className="icon iconfont icon-back_to_top" />
      </div>
    </Container>
  );
};
