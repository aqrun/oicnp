import { Result, Button } from 'antd';
import { r } from '@/utils';
import { Container } from './index.styled';

export default function NoAuth() {
  return (
    <Container>
      <Result
        status="403"
        title="403"
        subTitle={'糟糕！无权限操作，请联系管理员 ^O^'}
        extra={
          <Button
            type="primary"
            onClick={() => {
              // navigate(r('/login'))
            }}
          >
            返回登录
          </Button>
        }
      />
    </Container>
  );
}
