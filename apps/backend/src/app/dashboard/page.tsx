'use client';

import Overview from './Overview';
import SalePercent from './SalePercent';
import TimeLine from './TimeLine';
import { Container } from './index.styled';

export default function DashboardPage(): JSX.Element {
  return (
    <Container>
      <Overview loading={false} />
      <SalePercent loading={false} />
      <TimeLine loading={false} />
    </Container>
  )
}
