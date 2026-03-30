'use client';

import { Result, Button } from 'antd';
import { useRouter } from 'next/navigation';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

export default function NotFound() {
  const router = useRouter();

  const handleBackHome = useMemoizedFn(() => {
    router.push('/');
  });

  return (
    <Container>
      <Result
        status="404"
        title="404"
        subTitle={'糟糕！页面不存在'}
        extra={
          <Button
            type="primary"
            onClick={() => {
              handleBackHome();
            }}
          >
            返回首页
          </Button>
        }
      />
    </Container>
  );
}
