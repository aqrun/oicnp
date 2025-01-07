import { useMemoizedFn } from 'ahooks';
import { Input } from 'antd';
import { CLASS_PREFIX } from '~/constants';

const { Search } = Input;

export interface SearchBoxProps {
  placeholder?: string;
  onSearch?: (value: string) => void;
}

export function SearchBox({
  placeholder,
  onSearch,
}: SearchBoxProps): JSX.Element {
  const handleSearch = useMemoizedFn((value: string) => {
    if (typeof onSearch === 'function') {
      onSearch(value);
    }
  });

  return (
    <Search
      placeholder={placeholder || '请输入关键词'}
      allowClear
      onSearch={handleSearch}
      className={`${CLASS_PREFIX}-filter-search-box`}
    />
  );
}
