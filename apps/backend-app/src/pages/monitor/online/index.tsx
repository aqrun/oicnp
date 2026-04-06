'use client';

import type { ReactElement } from "react";

import OnlineList from './List';
import { Container } from './index.styled';

export default function OnlinePage(): ReactElement {
  return (
    <Container>
      <OnlineList />
    </Container>
  );
}