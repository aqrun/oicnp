import { useEffect } from 'react';
import { useParams } from 'next/navigation';

export interface NodeDetailPageProps {
  nid?: string;
  vid?: string;
}

export const NodeDetailPage: React.FC<NodeDetailPageProps> = ({
  nid,
  vid,
}) => {
  const params = useParams();

  useEffect(() => {
    console.log('page-data-params:', params);
  });

  return (
    <div>
      node detail page
    </div>
  );
};
