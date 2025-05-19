import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '@/components';
import NoteForm from '../NoteForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../List/useListStore';
import {
  usePermissionTree,
  usePermissionTreeStore,
} from '@/components/PermissionTree'
import {
  NoteModel,
  DescribeCreateNote,
  DescribeCreateNoteRequestParams,
} from '@/services';

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);
  const setPermissionTreeState = usePermissionTreeStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const [form] = Form.useForm<NoteModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateNoteRequestParams = {
        title: values?.title,
        content: values?.content,
      };

      const res = await DescribeCreateNote(params);

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

  const fetchInitialData = useMemoizedFn(async () => {
    setPermissionTreeState({
      checkedKeys: undefined,
    });
    await fetchPermissionTree();
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

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

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
      width={640}
      hasOk={contentType !== 'success'}
      okButtonProps={{
        loading,
      }}
    >
      {content}
    </Modal>
  );
}
