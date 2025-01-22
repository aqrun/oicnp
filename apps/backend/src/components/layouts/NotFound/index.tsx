import { Result, Button } from 'antd';
import { r } from '@/utils';
import { Container } from './index.styled';

export default function NotFound() {

  return (
    <Container>
      <Result
        status="404"
        title="404"
        subTitle={'糟糕！页面不存在'}
        extra={
          <Button
            type="primary"
            onClick={() => {}}
          >
            返回首页
          </Button>
        }
      />
    </Container>
  );
}
