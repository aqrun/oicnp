import Overview from './Overview';
import SalePercent from './SalePercent';
import TimeLine from './TimeLine';
import { Container } from './index.styled';

/**
 * 仪表盘
 */
export default function Dashboard(): JSX.Element {
  return (
    <Container>
      <Overview loading={false} />
      <SalePercent loading={false} />
      <TimeLine loading={false} />
    </Container>
  );
}