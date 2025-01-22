import {
  PlusCircleFilled,
  SearchOutlined,
} from '@ant-design/icons';
import {
  Input,
  theme,
} from 'antd';

export function SearchInput (): JSX.Element {
  const { token } = theme.useToken();
  return (
    <div
      aria-hidden
      key="SearchOutlined"
      onMouseDown={(e) => {
        e.stopPropagation();
        e.preventDefault();
      }}
      style={{
        display: 'flex',
        alignItems: 'center',
        marginInlineEnd: 24,
      }}
    >
      <Input
        bordered={false}
        placeholder="搜索方案"
        prefix={
          <SearchOutlined
            style={{
              color: token.colorTextLightSolid,
            }}
          />
        }
        style={{
          borderRadius: 4,
          marginInlineEnd: 12,
          backgroundColor: token.colorBgTextHover,
        }}
      />
      <PlusCircleFilled
        style={{
          color: token.colorPrimary,
          fontSize: 24,
        }}
      />
    </div>
  );
}
