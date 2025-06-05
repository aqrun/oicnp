'use client';

import {
  Button,
  Form,
  Input,
  FormInstance,
  Select,
  DatePicker,
} from 'antd';
import { TagInput } from '@/components';
import type { FormProps } from 'antd';
import {
  NodeModel,
  CategoryModel,
} from '@/services';
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
  categories?: CategoryModel[];
  categoryLoading?: boolean;
  onTagChange?: (tags: string[]) => void;
}

export default function NodeForm({
  onFinish,
  loading,
  isEdit,
  node,
  showSubmit,
  form,
  disabled,
  categories,
  categoryLoading,
  onTagChange,
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
        wrapperCol={{ span: 24 }}
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
          label="分类"
          name="categoryIds"
        >
          <Select
            mode="multiple"
            options={categories?.map(item => ({
              label: item?.catName,
              value: item?.catId,
            }))}
            loading={categoryLoading}
            allowClear
          />
        </Form.Item>
        <Form.Item<FieldType>
          label="标签"
          name="tagIds"
        >
          <TagInput
            onChange={onTagChange}
          />
        </Form.Item>
        <Form.Item<FieldType>
          label="摘要"
          name="summary"
        >
          <Input.TextArea rows={2} />
        </Form.Item>
        <Form.Item<FieldType>
          label="内容"
          name="body"
        >
          <Input.TextArea rows={10} />
        </Form.Item>
        <Form.Item<FieldType>
          label="发布时间"
          name="publishedAt"
        >
          <DatePicker
            showTime
          />
        </Form.Item>
        <Form.Item<FieldType>
          label="创建时间"
          name="createdAt"
        >
          <DatePicker
            showTime
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