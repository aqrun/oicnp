'use client';

import CronsList from './List';
import { Container } from './index.styled';

export default function OnlinePage(): JSX.Element {
  return (
    <Container>
      <CronsList />
    </Container>
  );
}
