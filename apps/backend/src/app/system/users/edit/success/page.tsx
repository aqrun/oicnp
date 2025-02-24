'use client';

import { Button, Result } from 'antd';
import { useRouter, useSearchParams } from 'next/navigation';
import { r } from '@/utils';

export default function CreateSuccess(): JSX.Element {
  const router = useRouter();
  const searchParams = useSearchParams();

  return (
    <Result
      status="success"
      title="更新成功"
      subTitle={(
        <>
          用户更新成功
        </>
      )}
      extra={[
        <Button
          type="primary"
          onClick={() => {
            router.push(r('/system/users'));
          }}
        >
          返回列表页
        </Button>,
        <Button
          type="default"
          onClick={() => {
            router.push(r(`/system/users/detail?uid=${searchParams.get('uid')}`));
          }}
        >
          查看详情
        </Button>
      ]}
    />
  );
}