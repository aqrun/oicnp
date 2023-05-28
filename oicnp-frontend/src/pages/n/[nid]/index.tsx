import { useEffect } from 'react';
import { NodeDetailPage } from '~/containers';
import { useParams } from 'next/navigation';

export default function NodeDetail () {
  const params = useParams();

  useEffect(() => {
    console.log('params---nid1111:', params);
  }, [params]);

  return (
    <NodeDetailPage />
  );
}