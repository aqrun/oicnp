import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  NoteModel,
  DescribeNoteDetail,
  DescribeNoteDetailRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const noteId = useViewStore(state => state.noteId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<NoteModel>();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      note: undefined,
    });
  });

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeNoteDetailRequestParams = {
      id: noteId,
    };
    const res = await DescribeNoteDetail(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const requests: Array<Promise<any>> = [
      fetchNote(),
    ];
    const allRes = await Promise.all(requests);

    setState({
      note: allRes?.[0]?.note,
    });
    setLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="查看笔记"
      open={visible}
      onCancel={handleCancel}
      confirmLoading={false}
      cancelText={'关闭'}
      destroyOnClose
      width={640}
      hasOk={false}
    >
      <Spin
        spinning={loading}
      >
        <Descriptions
          items={items}
          column={2}
          layout="vertical"
        />
      </Spin>
    </Modal>
  );
}
