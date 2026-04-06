import { Result } from 'antd';
import type { ReactElement } from 'react';

export interface CreateSuccessProp {
  title?: string;
}

export default function CreateSuccess({
  title,
}: CreateSuccessProp): ReactElement {
  return (
    <>
      <Result
        status="success"
        title="部门创建成功"
        subTitle={`部门 ${title} 创建成功`}
      />
    </>
  );
}
