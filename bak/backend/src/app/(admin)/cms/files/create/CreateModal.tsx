import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '@/components';
import FileForm from '../FileForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import {
  FileFieldType,
  DescribeCreateFile,
  DescribeCreateFileRequestParams,
  UploadFileRes,
} from '@/services';

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [uploadedFile, setUploadedFile] = useState<UploadFileRes | undefined>(undefined);

  const [form] = Form.useForm<FileFieldType>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateFileRequestParams = {
        fileId: uploadedFile?.id,
        filename: values?.filename,
        uri: values?.uri,
        storage: values?.storage,
        mime: values?.mime,
        status: "1",
      };

      const res = await DescribeCreateFile(params);

      if (res) {
        setState({
          contentType: 'success',
        });
      }
    } catch(err) {
      console.log('ERR: ', err);
    }

    setLoading(false);
  });

  const handleCancel = useMemoizedFn(() => {
    if (contentType === 'success') {
      form.resetFields();
      setUploadedFile(undefined);
      setListState({
        refreshToken: Date.now().toString(),
      });
    }

    setState({
      visible: false,
      contentType: 'normal',
    });
  });

  const handleUpload = useMemoizedFn((file: UploadFileRes) => {
    setUploadedFile(file);
  });

  const init = useMemoizedFn(async () => {
    setUploadedFile(undefined);
  });

  let content = (
    <FileForm
      form={form}
      loading={loading}
      onUploadChange={handleUpload}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('filename')}
      />
    );
  }

  useEffect(() => {
    if (visible) {
      init();
    }
  }, [visible]);

  return (
    <Modal
      title="创建文件"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
      cancelText={contentType !== 'success' ? '取消' : '关闭'}
      destroyOnClose
      width={840}
      hasOk={contentType !== 'success'}
      maskClosable={false}
      okButtonProps={{
        loading,
      }}
    >
      {content}
    </Modal>
  );
}
