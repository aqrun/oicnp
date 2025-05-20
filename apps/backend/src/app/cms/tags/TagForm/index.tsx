'use client';

import { Button, Form, Input, FormInstance } from 'antd';
import type { FormProps } from 'antd';
import { TagModel } from '@/services';
import { Container } from './index.styled';

type FieldType = TagModel;

export interface TagFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  tag?: TagModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<TagModel>;
  disabled?: boolean;
}

export default function NoteForm({
  onFinish,
  loading,
  isEdit,
  tag,
  showSubmit,
  form,
  disabled,
}: TagFormProps): JSX.Element {
  const initialValues: FieldType = {
    tagName: '',
    ...(tag || {}),
  };

  return (
    <Container>
      <Form
        name="basic"
        initialValues={initialValues}
        onFinish={onFinish}
        autoComplete="off"
        layout="vertical"
        form={form}
        disabled={disabled}
      >
        <Form.Item<FieldType>
          label="标签名称"
          name="tagName"
          rules={[{ required: true, message: '请输入标签名称！' }]}
        >
          <Input />
        </Form.Item>

        {showSubmit && (
          <Form.Item label={null}>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
            >
              {isEdit ? '更新' : '创建'}
            </Button>
          </Form.Item>
        )}
      </Form>
    </Container>
  );
}