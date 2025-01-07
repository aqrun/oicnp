import { Result, Button } from 'antd';
import { useNavigate } from 'react-router';
import { r } from '~/utils';
import { Container } from './index.styled';

export default function NotFound() {
  const navigate = useNavigate();

  return (
    <Container>
      <Result
        status="404"
        title="404"
        subTitle={'糟糕！页面不存在'}
        extra={
          <Button
            type="primary"
            onClick={() => navigate(r(''))}
          >
            返回首页
          </Button>
        }
      />
    </Container>
  );
}
