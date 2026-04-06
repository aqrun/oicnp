'use client';

import type { ReactElement } from "react";

import { useState } from 'react';
import {   Button, Form, Input, FormInstance, Select, } from 'antd';
import { FormProps } from 'antd';
import {   FileFieldType, UploadFileRes } from "@repo/apis";
import { useMemoizedFn } from 'ahooks';
import { callFn } from '#src/utils';
import dayjs from 'dayjs';
import FileUploader from './FileUploader';
import { Container } from './index.styled';

type FieldType = FileFieldType;

export interface TagFormProps {
  onFinish?: FormProps<FileFieldType>['onFinish'];
  isEdit?: boolean;
  file?: UploadFileRes;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<FieldType>;
  disabled?: boolean;
  onUploadChange?: (file: UploadFileRes) => void;
}

export default function FileForm({
  onFinish,
  loading,
  isEdit,
  file,
  showSubmit,
  form,
  disabled,
  onUploadChange,
}: TagFormProps): ReactElement {
  const [storage, setStorage] = useState<string>('local');

  const handleFileUpload = useMemoizedFn((paramFile: UploadFileRes) => {
    if (paramFile) {
      form?.setFieldsValue({
        filename: paramFile?.name,
        uri: paramFile?.uri,
        mime: paramFile?.mime,
      });
    }

    callFn(onUploadChange, paramFile);
  });

  const getInitialValues = () => {
    const data: FileFieldType = {
      filename: undefined,
      uri: undefined,
      storage: 'local',
      mime: undefined,
      status: undefined,
    }

    if (file?.createdAt) {
      data.createdAt = dayjs(file?.createdAt);
    }

    return data;
  };
  const initialValues = getInitialValues();

  const handleStorageChange = (value: string) => {
    setStorage(value);
  };

  return (
    <Container>
      <FileUploader
        file={file}
        storage={storage}
        onChange={handleFileUpload}
      />
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
            <Input
              placeholder="请输入文件名"
            />
          </Form.Item>
          <Form.Item<FieldType>
            label="文件路径"
            name="uri"
          >
            <Input
              placeholder="请输入文件路径"
              disabled={Boolean(file)}
            />
          </Form.Item>
          <Form.Item<FieldType>
            label="图床地址"
            name="link"
          >
            <Input
              placeholder="请输入图床地址"
            />
          </Form.Item>
          <Form.Item<FieldType>
            label="存储"
            name="storage"
          >
            <Select
              placeholder="请选择存储"
              onChange={handleStorageChange}
              disabled={Boolean(file)}
              options={[
                {
                  label: '本地',
                  value: 'local',
                },
                {
                  label: 'OSS',
                  value: 'oss',
                },
              ]}
              loading={false}
              allowClear
            />
          </Form.Item>
          <Form.Item<FieldType>
            label="mime"
            name="mime"
          >
            <Input
              placeholder="请输入mime"
              disabled={Boolean(file)}
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
      </div>
    </Container>
  );
}