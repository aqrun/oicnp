import React, { useEffect, useRef, useMemo, useState } from 'react';
import {
  Node,
} from '../../typings';

export interface PagerItemProps {
  node: Node;
  className?: string;
}

export const PagerItem: React.FC<PagerItemProps> = ({
  node,
  className,
}) => {
  const [marginTop, setMarginTop] = useState(0);
  const rnRef = useRef<HTMLDivElement>(null);
  const nRef = useRef<HTMLDivElement>(null);

  const sectionStyle = useMemo(() => {
    return {
      marginTop: `${marginTop}px`
    };
  }, [marginTop]);

  useEffect(() => {
    const n = nRef.current?.offsetHeight || 0;
    const rn = rnRef.current?.offsetHeight || 0;
    setMarginTop((rn - n ) / 2);
  }, []);

  return (
    <div className={`read-next-item ${className || ''}`} ref={rnRef}>
      <a href={`/blog/${node?.nid}/${node?.vid}`} className="read-next-link"></a>
      <section ref={nRef} style={sectionStyle}>
        <span>{node?.title}</span>
        <p>{node?.nodeBody?.summary}</p>
      </section>
      {node?.cover && (
        <>
          <div className="filter"></div>
          <img src={node?.cover} alt="" />
        </>
      )}
    </div>
  );
}