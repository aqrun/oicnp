import {
  Modal as AntModal,
  ModalProps as AntModalProps,
} from 'antd';
import ModalFooter from './ModalFooter';

export interface ModalProps extends AntModalProps {
  hasOk?: boolean;
  hasCancel?: boolean;
}

export function Modal(props: React.PropsWithChildren<ModalProps>) {
  const {
    okText,
    cancelText,
    loading,
    onOk,
    onCancel,
    hasOk,
    hasCancel,
    children,
  } = props;

  return (
    <AntModal
      width={640}
      {...props}
      footer={(
        <ModalFooter
          okText={okText}
          cancelText={cancelText}
          loading={loading}
          onOk={onOk}
          onCancel={onCancel}
          hasOk={hasOk}
          hasCancel={hasCancel}
        />
      )}
    >
      {children}
    </AntModal>
  );
}