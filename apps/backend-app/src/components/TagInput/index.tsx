import { Select } from 'antd';
import { useMemoizedFn } from 'ahooks';

export interface TagInputProps {
  defaultTags?: string[];
  onChange?: (tags: string[]) => void;
}

export function TagInput({ defaultTags, onChange }: TagInputProps) {
  const handleChange = useMemoizedFn((values: string[]) => {
    onChange?.(values);
  });

  return (
    <Select
      mode="tags"
      allowClear
      style={{ width: '100%' }}
      placeholder="请输入标签并回车"
      defaultValue={defaultTags}
      onChange={handleChange}
      options={[]}
    />
  );
}
