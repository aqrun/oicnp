'use client';

import {
  Upload,
  Button,
} from 'antd';
import type {
  UploadFile,
  UploadProps,
  GetProp,
} from 'antd';
import {
  Icon,
} from '@/components';
import { r } from '@/utils';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

type FileType = Parameters<GetProp<UploadProps, 'beforeUpload'>>[0];

export default function FileUploader() {
  const handlePreview = useMemoizedFn(async (file: UploadFile) => {
    console.log(file);
  });

  const handleChange: UploadProps['onChange'] = useMemoizedFn(async (info) => {
    if (info?.file?.percent !== 100 || info?.file?.status !== 'done') return;

    const src = await new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = () => {
        resolve(reader.result as string);
      };
      reader.readAsDataURL(info.file.originFileObj as FileType);
    });

    console.log('src---', src);
    console.log(info);
  });

  const uploadData = useMemoizedFn((file: UploadFile) => {
    return {
      fileName: file?.name,
      name: 'alex',
      age: 18,
    }
  });

  return (
    <Container>
      <div className="oic-uploader-w">
        <Upload
          action={`${r('v1/file/upload')}`}
          onPreview={handlePreview}
          onChange={handleChange}
          maxCount={1}
          listType="text"
          data={uploadData}
        >
          <Button>
            <Icon icon="UploadOutlined" />
            上传文件
          </Button>
        </Upload>
      </div>
    </Container>
  );
}
