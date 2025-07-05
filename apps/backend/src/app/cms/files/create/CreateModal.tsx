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

  const [form] = Form.useForm<FileFieldType>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateFileRequestParams = {
        fileName: values?.fileName,
        uri: values?.uri,
        storage: values?.storage,
        mime: values?.mime,
        status: values?.status,
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
      setListState({
        refreshToken: Date.now().toString(),
      });
    }

    setState({
      visible: false,
      contentType: 'normal',
    });
  });

  const init = useMemoizedFn(async () => {
    // const requests = [
      
    // ] as const;
    // const allRes = await Promise.all(requests);
    // setState({
    //   categories: allRes[0].categories,
    // });
  });

  let content = (
    <FileForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('fileName')}
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
