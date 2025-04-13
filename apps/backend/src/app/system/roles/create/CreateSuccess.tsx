import { Result } from 'antd';

export interface CreateSuccessProp {
  roleName?: string;
}

export default function CreateSuccess({
  roleName,
}: CreateSuccessProp): JSX.Element {
  return (
    <>
      <Result
        status="success"
        title="角色创建成功"
        subTitle={`角色 ${roleName} 创建成功`}
      />
    </>
  );
}
