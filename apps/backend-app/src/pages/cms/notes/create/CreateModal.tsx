import { useState } from 'react';
import { Form } from 'antd';
import { Modal } from '#src/components';
import NoteForm from '../NoteForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import {   NoteModel, DescribeCreateNoteRequestParams } from "@repo/apis";
import { noteApis } from "#src/api";

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const [form] = Form.useForm<NoteModel>();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateNoteRequestParams = {
        title: values?.title,
        content: values?.content,
      };

      const res = await noteApis.DescribeCreateNote(params);

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

  let content = (
    <NoteForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        title={form.getFieldValue('title')}
      />
    );
  }

  return (
    <Modal
      title="创建笔记"
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
