import { CLASS_PREFIX } from '~/constants';
import cls from 'clsx';
import { CreateButton } from './CreateButton';
import { RefreshButton } from './RefreshButton';
import { SearchBox } from './SearchBox';
import { Container } from './index.styled';

export interface FiltersProps {
  createLabel?: string;
  placeholder?: string;
  onSearch?: (value: string) => void;
  onCreate?: () => void;
  onRefresh?: () => void;
}

/**
 * 仪表盘
 */
export function Filters({
  createLabel,
  placeholder,
  onSearch,
  onCreate,
  onRefresh,
}: FiltersProps): JSX.Element {
  return (
    <Container>
      <div className={cls(`${CLASS_PREFIX}-filter-left`)}>
        {Boolean(onCreate) && (
          <CreateButton
            label={createLabel}
          />
        )}
        {Boolean(onSearch) && (
          <SearchBox
            placeholder={placeholder}
            onSearch={onSearch}
          />
        )}
      </div>
      <div className={cls(`${CLASS_PREFIX}-filter-right`)}>
        {Boolean(onRefresh) && (
          <RefreshButton
            onRefresh={onRefresh}
          />
        )}
      </div>
    </Container>
  );
}