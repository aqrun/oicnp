import { useRef } from 'react';
import { useMemoizedFn, useDebounceFn } from 'ahooks';
import { useGlobalState } from '@/context';
import { ModalFuncProps } from 'antd/lib/modal';

export default function useModal() {
  const {
    modal,
  } = useGlobalState();
  const ref = useRef<any>();

  const { run: showError } = useDebounceFn((props: ModalFuncProps) => {
    if (ref.current) {
      ref.current.destroy();
    }

    ref.current = modal?.error({
      ...props,
    });
  }, { wait: 300 });

  return {
    showError,
  };
}
