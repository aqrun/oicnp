'use client';

import { useState, useEffect } from 'react';
import { getToken } from '@/actions/getUser';
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

export default function FileUploader() {
  const [token, setToken] = useState('');

  const headers = {
    'Authorization': `Bearer ${token}`,
  };

  const handleChange: UploadProps['onChange'] = useMemoizedFn(async (info) => {
    if (info?.file?.percent !== 100 || info?.file?.status !== 'done') return;
    console.log(info);
  });

  const uploadData = useMemoizedFn((file: UploadFile) => {
    return {
      name: file?.name,
      size: file?.size,
      type: file?.type,
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
      </div>
    </Container>
  );
}
