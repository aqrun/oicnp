'use client';

import { useEffect } from 'react';
import { App } from 'antd';
import { useAppStore } from '@/stores';
import { useMemoizedFn } from 'ahooks';
import { FailModel } from '@/types';

/**
 * 全局错误信息显示
 */
export function ErrorHandler(): JSX.Element {
  const { modal } = App.useApp();
  const errors = useAppStore((state) => state.errors);
  const setState = useAppStore((state) => state.setState);

  const showError = useMemoizedFn((error: FailModel) => {
    modal.error({
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
    });
  });

  useEffect(() => {
    if (errors?.length) {
      showError(errors?.[0]);
      setState({
        errors: [],
      });
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [errors]);

  return <></>;
}
