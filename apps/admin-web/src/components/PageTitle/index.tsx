import {
  CLASS_PREFIX,
} from '~/constants';
import { Container } from './index.styled';

export interface PageTitleProps {
  title?: React.ReactNode;
}

export function PageTitle({
  title,
}: PageTitleProps): JSX.Element {
  return (
    <Container
      className={`${CLASS_PREFIX}-pageTitle-container`}
    >
      <h2>
        {title}
      </h2>
    </Container>
  );
}
