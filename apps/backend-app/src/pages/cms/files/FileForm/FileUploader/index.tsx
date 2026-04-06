'use client';

import { useState } from 'react';
import type { UploadFileRes } from "@repo/apis";
import { callFn} from '#src/utils';
import {
  Upload,
  Button,
  Image,
} from 'antd';
import type {
  UploadFile,
  UploadProps,
} from 'antd';
import {
  Icon,
} from '#src/components';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

export interface FileUploaderProps {
  file?: UploadFileRes;
  storage: string;
  onChange?: (file: UploadFileRes) => void;
}

export default function FileUploader({
  file,
  storage,
  onChange,
}: FileUploaderProps) {
  const [imgUrl, setImgUrl] = useState('');

  const handleChange: UploadProps['onChange'] = useMemoizedFn(async (info) => {
    if (info?.file?.percent !== 100 || info?.file?.status !== 'done') return;
    const fileData = info.file.response?.data?.file as UploadFileRes;
    setImgUrl(fileData?.link || fileData?.url);
    callFn(onChange, fileData);
  });

  const uploadData = useMemoizedFn((paramFile: UploadFile) => {
    return {
      name: paramFile?.name,
      size: paramFile?.size,
      type: paramFile?.type,
      storage,
    }
  });

  return (
    <Container>
      <div className="oic-uploader-w">
        {!Boolean(file) && (
          <Upload
            action={`${import.meta.env.VITE_API_BASE_URL}/file/upload`}
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
        )}

        {Boolean(file || imgUrl) && (
          <div className="oic-uploader-img">
            <Image
              src={file?.url || imgUrl}
              alt={file?.name || file?.uri || ''}
              width={400}
              height={300}
            />
          </div>
        )}
      </div>
    </Container>
  );
}
