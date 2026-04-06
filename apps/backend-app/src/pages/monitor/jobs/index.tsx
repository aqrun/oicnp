'use client';

import type { ReactElement } from "react";

import CronsList from './List';
import { Container } from './index.styled';

export default function OnlinePage(): ReactElement {
  return (
    <Container>
      <CronsList />
    </Container>
  );
}