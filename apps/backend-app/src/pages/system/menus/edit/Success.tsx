import type { ReactElement } from "react";
import { Result } from 'antd';

export interface SuccessProps {
  roleName?: string;
}

export default function Success({
  roleName,
}: SuccessProps): ReactElement {
  return (
    <>
      <Result
        status="success"
        title="角色更新成功"
        subTitle={`角色 ${roleName} 更新成功`}
      />
    </>
  );
}
