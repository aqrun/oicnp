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
      title="创建成功"
      subTitle={(
        <>
          用户: {searchParams.get('nickname')} 创建成功
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
          onClick={() => {
            router.push(r('/system/users/create'));
          }}
        >继续创建</Button>,
      ]}
    />
  );
}