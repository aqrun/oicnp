import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '#src/components';
import NoteForm from '../NoteForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../List/useListStore';
import {   NoteModel, DescribeNoteDetailRequestParams, DescribeUpdateNoteRequestParams } from "@repo/apis";
import { noteApis } from "#src/api";

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const noteId = useEditStore(state => state.noteId);
  const note = useEditStore(state => state.note);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<NoteModel>();

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeNoteDetailRequestParams = {
      id: noteId,
    };
    const res = await noteApis.DescribeNoteDetail(params);
    
    setState({
      note: res.note,
    });
    
    form.setFieldsValue({
      title: res?.note?.title,
      content: res?.note?.content,
    });
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateNoteRequestParams = {
        id: noteId,
        title: values?.title,
        content: values?.content,
      };

      const res = await noteApis.DescribeUpdateNote(params);

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
          title={form.getFieldValue('title')}
        />
      );
    } else {
      return (
        <NoteForm
          form={form}
          loading={loading}
          note={note}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    await fetchNote();
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑角色"
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
