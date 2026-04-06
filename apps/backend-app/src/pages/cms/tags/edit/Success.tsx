import type { ReactElement } from "react";
import { Result } from 'antd';

export interface SuccessProps {
  title?: string;
}

export default function Success({
  title,
}: SuccessProps): ReactElement {
  return (
    <>
      <Result
        status="success"
        title="标签更新成功"
        subTitle={`标签 ${title} 更新成功`}
      />
    </>
  );
}
