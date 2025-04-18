import { callFn } from '@/utils';
import { useMemoizedFn } from 'ahooks';
import {
  Button,
  ModalProps as AntModalProps,
} from 'antd';
import type { ButtonProps } from 'antd/lib/button';

export interface ModalFooterProps {
  okText?: React.ReactNode;
  cancelText?: React.ReactNode;
  loading?: boolean;
  onOk?: AntModalProps['onOk'];
  onCancel?: AntModalProps['onCancel'];
  hasOk?: boolean;
  hasCancel?: boolean;
  okButtonProps?: ButtonProps;
}

export default function ModalFooter({
  okText,
  cancelText,
  loading,
  onOk,
  onCancel,
  hasOk = true,
  hasCancel = true,
  okButtonProps,
}: ModalFooterProps) {

  const handleOk = useMemoizedFn(() => {
    callFn(onOk);
  });

  const handleCancel = useMemoizedFn(() => {
    callFn(onCancel);
  });

  return (
    <div className="flex items-center justify-end">
      {hasOk && (
        <Button
          type={'primary'}
          loading={loading}
          onClick={handleOk}
          {...(okButtonProps || {})}
        >
          {okText || '确定'}
        </Button>
      )}
      {hasCancel && (
        <Button
          onClick={handleCancel}
          className="ml-2"
        >
          {cancelText || '取消'}
        </Button>
      )}
    </div>
  );
}
