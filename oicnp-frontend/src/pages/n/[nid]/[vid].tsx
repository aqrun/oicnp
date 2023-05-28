import { useEffect } from 'react';
import { NodeDetailPage } from '~/containers';
import { useParams } from 'next/navigation';

{/* @ts-expect-error Async Server Component */}
const NodeVidDetail: React.FC<NodeVidDetailProps> = () => {
  const params = useParams();

  useEffect(() => {
    console.log('params---vd:', params);
  });

  return (
    <NodeDetailPage />
  );
};

export default NodeVidDetail;