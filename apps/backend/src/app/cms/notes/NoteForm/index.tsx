'use client';

import { Button, Form, Input, FormInstance } from 'antd';
import type { FormProps } from 'antd';
import { NoteModel } from '@/services';
import MDEditor from '@uiw/react-md-editor';
import { Container } from './index.styled';
type FieldType = NoteModel;

export interface NoteFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  note?: NoteModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<NoteModel>;
  disabled?: boolean;
}

export default function NoteForm({
  onFinish,
  loading,
  isEdit,
  note,
  showSubmit,
  form,
  disabled,
}: NoteFormProps): JSX.Element {
  const initialValues: FieldType = {
    title: '',
    content: '',
    ...(note || {}),
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
          label="标题"
          name="title"
          rules={[{ required: true, message: '请输入标题！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="内容"
          name="content"
          rules={[{ required: true, message: '请输入内容！' }]}
        >
          <MDEditor
            value={note?.content || ''}
            onChange={(value) => {
              console.log(value);
              console.log('-----', form?.getFieldsValue())
            }}
            style={{ minHeight: 450 }}
          />
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