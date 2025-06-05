'use client';

import { useState, useEffect } from 'react';
import { Tag, Input } from 'antd';
import { useTagsStore } from './useTagsStore';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

export interface TagInputProps {
  defaultTags?: string[];
  onChange?: (tags: string[]) => void;
}

/**
 * 标签输入
 */
export default function TagInput({
  defaultTags,
  onChange,
}: TagInputProps) {
  const values = useTagsStore(state => state.values);
  const setState = useTagsStore(state => state.setState);
  const [inputValue, setInputValue] = useState<string>('');

  const removeTag = useMemoizedFn((tag: string) => {
    setState({
      values: values?.filter(v => v !== tag),
    });
  });

  const handleChange = useMemoizedFn((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.currentTarget.value;
    setInputValue(value);
  });

  const handleEnter = useMemoizedFn((e: React.KeyboardEvent<HTMLInputElement>) => {
    const value = e.currentTarget.value;
    if (value) {
      const newValues = Array.from(new Set([...(values || []), value]));
      setState({
        values: newValues,
      });
      onChange?.(newValues);
      setInputValue('');
    }
  });

  useEffect(() => {
    if (defaultTags?.length) {
      setState({
        values: defaultTags,
      });
    }
  }, [defaultTags]);

  return (
    <Container className="rounded-md border border-gray-300 p-2">
      <div>
        {values?.map(item => (
          <Tag
            key={item}
            closable
            onClose={() => {
              removeTag(item);
            }}
            className="mb-1"
          >
            {item}
          </Tag>
        ))}
        <Input
          placeholder="请输入标签"
          onPressEnter={handleEnter}
          value={inputValue}
          onChange={handleChange}
          className="border-none inline-block w-auto"
        />
      </div>
    </Container>
  );
}