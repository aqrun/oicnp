import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import FileForm from '../FileForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {
  DescribeFileDetailRequestParams,
  DescribeUpdateFile,
  DescribeUpdateFileRequestParams,
  FileFieldType,
  useFetchFile,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const fileId = useEditStore(state => state.fileId);
  const file = useEditStore(state => state.file);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<FileFieldType>();

  const {
    fetchFile,
  } = useFetchFile();

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeFileDetailRequestParams = {
      fileId,
    };
    const request = [
      fetchFile(params),
    ] as const;
    const allRes = await Promise.all(request);
    
    setState({
      file: allRes?.[0]?.file,
    });

    form.setFieldsValue({
      fileId: allRes?.[0]?.file?.fileId,
      filename: allRes?.[0]?.file?.filename,
      uri: allRes?.[0]?.file?.uri,
      storage: allRes?.[0]?.file?.storage,
      mime: allRes?.[0]?.file?.mime,
      status: allRes?.[0]?.file?.status,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateFileRequestParams = {
        fileId,
        filename: values?.filename,
        uri: values?.uri,
        storage: values?.storage,
        mime: values?.mime,
        status: values?.status,
      };

      const res = await DescribeUpdateFile(params);

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
      setListState({
        refreshToken: Date.now().toString(),
      });
    }

    setState({
      visible: false,
      contentType: 'normal',
    });
  });

  const getContent = () => {
    if (initLoading) {
      return (
        <Skeleton active />
      );
    } else if (contentType === 'success') {
      return (
        <Success
          title={form.getFieldValue('filename')}
        />
      );
    } else {
      return (
        <FileForm
          form={form}
          loading={loading}
          file={file}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    const requests = [
      fetchNote(),
    ];
    await Promise.all(requests);
    
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑文件"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="更新"
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
