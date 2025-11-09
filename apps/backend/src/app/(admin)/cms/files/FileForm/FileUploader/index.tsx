'use client';

import { useState, useEffect } from 'react';
import { getToken, UploadFileRes } from '@/services';
import Image from 'next/image';
import { useAppStore } from '@/stores/useAppStore';
import { callFn} from '@/utils';
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
  /**
   * 当前编辑的文件信息
   */
  file?: UploadFileRes;
  storage: string;
  onChange?: (file: UploadFileRes) => void;
}

export default function FileUploader({
  file,
  storage,
  onChange,
}: FileUploaderProps) {
  const setAppState = useAppStore(state => state.setState);
  const [token, setToken] = useState('');
  const [imgUrl, setImgUrl] = useState('');

  const headers = {
    'Authorization': `Bearer ${token}`,
  };

  const handleChange: UploadProps['onChange'] = useMemoizedFn(async (info) => {
    if (info?.file?.percent !== 100 || info?.file?.status !== 'done') return;
    const fileData = info.file.response?.data?.file as UploadFileRes;
    // 优先使用图床地址
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

  const init = useMemoizedFn(async () => {
    const token = await getToken();

    if (!token) {
      setAppState({
        errors: [{
          code: '401',
          message: '用户未登录',
        }],
      });
    }

    setToken(token || '');
  });

  useEffect(() => {
    init();
  }, []);

  return (
    <Container>
      <div className="oic-uploader-w">
        {!Boolean(file) && (
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
