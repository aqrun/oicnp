'use client';

import { useEffect, useRef } from 'react';
import { App } from 'antd';
import { useAppStore } from '@/stores';
import { useMemoizedFn } from 'ahooks';
import { FailModel } from '@/types';
import ModalFooter from '@/components/Modal/ModalFooter';
import { logoutAction } from '@/actions/logout';

/**
 * 全局错误信息显示
 */
export function ErrorHandler(): JSX.Element {
  const { modal } = App.useApp();
  const errors = useAppStore((state) => state.errors);
  const setState = useAppStore((state) => state.setState);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const instanceRef = useRef<any>();

  /**
   * 跳转登录界面
   */
  const handleLogout = useMemoizedFn(async () => {
    setState({
      errors: [],
    });
    await logoutAction();
    if (instanceRef.current) {
      instanceRef.current?.destroy?.();
      instanceRef.current = undefined;
    }
  });

  const handleClose = useMemoizedFn(() => {
    setState({
      errors: [],
    });
    if (instanceRef.current) {
      instanceRef.current?.destroy?.();
      instanceRef.current = undefined;
    }
  });

  const showError = useMemoizedFn((error: FailModel) => {
    let footer = (
      <ModalFooter
        hasOk={false}
        cancelText="关闭"
        onCancel={handleClose}
      />
    );

    if (error?.code === 'UserNeedLogin') {
      footer = (
        <ModalFooter
          okText="跳转登录"
          cancelText="已登录"
          onOk={async () => {
            await handleLogout();
          }}
          onCancel={handleClose}
        />
      );
    }

    if (!instanceRef.current) {
      instanceRef.current = modal.error({
        title: '请求失败',
        content: (
          <>
            错误码: {error?.code}<br/>
            错误信息：{error?.message}<br/>
            {Boolean(error?.action) && (
              <span>
                操作：{error?.action}<br/>
              </span>
            )}
            {Boolean(error?.requestId) && (
              <span>
                RequestId：{error?.requestId}<br/>
              </span>
            )}
          </>
        ),
        footer,
      });

      setState({
        errors: [],
      });
    }
  });

  useEffect(() => {
    if (errors?.length) {
      showError(errors?.[0]);
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [errors]);

  return <></>;
}
