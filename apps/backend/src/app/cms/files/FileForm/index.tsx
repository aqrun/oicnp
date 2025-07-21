'use client';

import {
  Button,
  Form,
  Input,
  FormInstance,
  Select,
} from 'antd';
import type { FormProps } from 'antd';
import {
  FileModel,
  FileFieldType,
} from '@/services';
import dayjs from 'dayjs';
import FileUploader from './FileUploader';
import { Container } from './index.styled';

type FieldType = FileFieldType;

export interface TagFormProps {
  onFinish?: FormProps<FileFieldType>['onFinish'];
  isEdit?: boolean;
  file?: FileModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<FieldType>;
  disabled?: boolean;
}

export default function FileForm({
  onFinish,
  loading,
  isEdit,
  file,
  showSubmit,
  form,
  disabled,
}: TagFormProps): JSX.Element {
  const getInitialValues = () => {
    const data: FileFieldType = {
      filename: '',
      uri: '',
      storage: '',
      mime: '',
      status: '',
    }

    if (file?.createdAt) {
      data.createdAt = dayjs(file?.createdAt);
    }

    return data;
  };
  const initialValues = getInitialValues();

  return (
    <Container>
      <FileUploader />
      <div className="oic-form-w">
        <Form
          name="basic"
          initialValues={initialValues}
          onFinish={onFinish}
          autoComplete="off"
          layout="vertical"
          form={form}
          disabled={disabled}
          wrapperCol={{ span: 24 }}
        >
          <Form.Item<FieldType>
            label="文件名"
            name="filename"
            rules={[{ required: true, message: '请输入文件名！' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item<FieldType>
            label="文件路径"
            name="uri"
            rules={[{ required: true, message: '请输入文件路径！' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item<FieldType>
            label="存储"
            name="storage"
          >
            <Select
              options={[]}
              loading={false}
              allowClear
            />
          </Form.Item>
          <Form.Item<FieldType>
            label="mime"
            name="mime"
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
      </div>
    </Container>
  );
}