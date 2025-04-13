import { Result } from 'antd';

export interface CreateSuccessProp {
  name?: string;
}

export default function CreateSuccess({
  name,
}: CreateSuccessProp): JSX.Element {
  return (
    <>
      <Result
        status="success"
        title="权限创建成功"
        subTitle={`权限 ${name} 创建成功`}
      />
    </>
  );
}
