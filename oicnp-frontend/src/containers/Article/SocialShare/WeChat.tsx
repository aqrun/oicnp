import React, { useEffect, useRef } from 'react';
import { useMemoizedFn } from 'ahooks';
import Script from 'next/script';

export interface WeChatProps {
  url: string;
}

export const WeChat: React.FC<WeChatProps> = ({
  url,
}) => {
  const win = window as any;
  const ref = useRef<HTMLDivElement>(null);
  const initSocialShare = useMemoizedFn(() => {
    new win.QRCode(ref?.current, {
      text: url || win.location.href,
      width: 100,
      height: 100,
    });
  });

  useEffect(() => {
    if (win.QRCode) {
      initSocialShare();
    }
  }, [win.QRCode]);

  return (
    <a
      key="wechat"
      className="social-share-icon icon-wechat"
      target="_blank"
      href="javscript:"
      rel="noreferrer"
    >
      <Script
        src="/assets/js/qrcode.min.js"
      />
      <div className="wechat-qrcode">
        <h4>分享到微信朋友圈</h4>
        <div className="qrcode" ref={ref} />
        <div className="help">
          <p>扫码后点击右上角</p>
          <p>将本文分享至朋友圈</p>
        </div>
      </div>
    </a>
  );
};
