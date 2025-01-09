import { CLASS_PREFIX } from '~/constants';
import cls from 'clsx';
import { CreateButton } from './CreateButton';
import { RefreshButton } from './RefreshButton';
import { SearchBox } from './SearchBox';
import { FilterValues } from '~/types';
import { Container } from './index.styled';

export interface FiltersProps {
  createLabel?: string;
  placeholder?: string;
  onSearch?: (value: FilterValues) => void;
  onChange?: (values: FilterValues, trigger?: string) => void;
  onCreate?: () => void;
  onRefresh?: () => void;
}

/**
 * 筛选组件
 */
export function Filters({
  createLabel,
  placeholder,
  onSearch,
  onChange,
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
            onChange={onChange}
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