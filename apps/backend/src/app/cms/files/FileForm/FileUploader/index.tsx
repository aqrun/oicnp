'use client';

import { useState, useEffect } from 'react';
import { getToken } from '@/actions/getUser';
import Image from 'next/image';
import {
  Upload,
  Button,
} from 'antd';
import type {
  UploadFile,
  UploadProps,
} from 'antd';
import {
  Icon,
} from '@/components';
import { API_URI } from '@/constants';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

export interface FileUploaderProps {
  storage: string;
}

export default function FileUploader({
  storage,
}: FileUploaderProps) {
  const [token, setToken] = useState('');
  const [imgUrl, setImgUrl] = useState('');

  const headers = {
    'Authorization': `Bearer ${token}`,
  };

  const handleChange: UploadProps['onChange'] = useMemoizedFn(async (info) => {
    if (info?.file?.percent !== 100 || info?.file?.status !== 'done') return;
    console.log(info);
    setImgUrl(info.file.response?.data?.file?.url);
  });

  const uploadData = useMemoizedFn((file: UploadFile) => {
    return {
      name: file?.name,
      size: file?.size,
      type: file?.type,
      storage,
    }
  });

  const init = useMemoizedFn(async () => {
    const token = await getToken();
    setToken(token || '');
  });

  useEffect(() => {
    init();
  }, []);

  return (
    <Container>
      <div className="oic-uploader-w">
        <Upload
          action={`${API_URI}/v1/file/upload`}
          onChange={handleChange}
          maxCount={1}
          listType="text"
          data={uploadData}
          headers={headers}
        >
          <Button>
            <Icon icon="UploadOutlined" />
            上传文件
          </Button>
        </Upload>

        {Boolean(imgUrl) && (
          <div className="oic-uploader-img">
            <Image
              src={imgUrl}
              alt="img"
              width={400}
              height={300}
            />
          </div>
        )}
      </div>
    </Container>
  );
}
