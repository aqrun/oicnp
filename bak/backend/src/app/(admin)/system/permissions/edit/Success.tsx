import { Result } from 'antd';

export interface SuccessProps {
  name?: string;
}

export default function Success({
  name,
}: SuccessProps): JSX.Element {
  return (
    <>
      <Result
        status="success"
        title="权限更新成功"
        subTitle={`权限 ${name} 更新成功`}
      />
    </>
  );
}
