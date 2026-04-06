'use client';

import type { ReactElement } from "react";

import { Button, Form, Input, FormInstance } from 'antd';
import { FormProps } from 'antd';
import { CategoryModel } from "@repo/apis";
import { Container } from './index.styled';

type FieldType = CategoryModel;

export interface CategoryFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  category?: CategoryModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<CategoryModel>;
  disabled?: boolean;
}

export default function NoteForm({
  onFinish,
  loading,
  isEdit,
  category,
  showSubmit,
  form,
  disabled,
}: CategoryFormProps): ReactElement {
  const initialValues: FieldType = {
    catName: '',
    ...(category || {}),
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
          label="父级"
          name="catPid"
          rules={[{ required: true, message: '请选择父级！' }]}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="标识"
          name="catVid"
          rules={[{ required: true, message: '请输入标识！' }]}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="名称"
          name="catName"
          rules={[{ required: true, message: '请输入名称！' }]}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="权重"
          name="weight"
          initialValue={0}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="描述"
          name="catDesc"
        >
          <Input.TextArea />
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