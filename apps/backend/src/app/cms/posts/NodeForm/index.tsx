'use client';

import { Button, Form, Input, FormInstance } from 'antd';
import type { FormProps } from 'antd';
import { NodeModel } from '@/services';
import { Container } from './index.styled';

type FieldType = NodeModel;

export interface TagFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  node?: NodeModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<NodeModel>;
  disabled?: boolean;
}

export default function NodeForm({
  onFinish,
  loading,
  isEdit,
  node,
  showSubmit,
  form,
  disabled,
}: TagFormProps): JSX.Element {
  const initialValues: FieldType = {
    title: '',
    ...(node || {}),
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
        wrapperCol={{ span: 10 }}
      >
        <Form.Item<FieldType>
          label="标识"
          name="vid"
          rules={[{ required: true, message: '请输入标识！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="标题"
          name="title"
          rules={[{ required: true, message: '请输入标题！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="内容"
          name="content"
          initialValue={0}
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