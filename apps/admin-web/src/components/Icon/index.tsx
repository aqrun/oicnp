import {} from '@ant-design/icons';
import { CLASS_PREFIX } from '~/constants';
import cls from 'clsx';
import {
  DashboardOutlined,
  UserOutlined,
  DesktopOutlined,
  LaptopOutlined,
  InboxOutlined,
  TeamOutlined,
  BulbOutlined,
  UsergroupAddOutlined,
  TagsOutlined,
  ClusterOutlined,
  SettingOutlined,
  DollarOutlined,
  UnlockOutlined,
  CaretUpOutlined,
  CaretDownOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import { Container } from './index.styled';

/**
 * 所有使用的icon可以在这里对应一次
 * 方便后端只返回icon字符
 */
const antIcons: Record<string, JSX.Element> = {
  DashboardOutlined: <DashboardOutlined />,
  DesktopOutlined: <DesktopOutlined />,
  LaptopOutlined: <LaptopOutlined />,
  InboxOutlined: <InboxOutlined/>,
  TeamOutlined: <TeamOutlined />,
  BulbOutlined: <BulbOutlined />,
  UsergroupAddOutlined: <UsergroupAddOutlined />,
  TagsOutlined: <TagsOutlined />,
  ClusterOutlined: <ClusterOutlined />,
  SettingOutlined: <SettingOutlined />,
  UserOutlined: <UserOutlined />,
  DollarOutlined: <DollarOutlined />,
  UnlockOutlined: <UnlockOutlined />,
  CaretUpOutlined: <CaretUpOutlined />,
  CaretDownOutlined: <CaretDownOutlined />,
  ReloadOutlined: <ReloadOutlined />,
};

export interface IconProps {
  icon: string;
  color?: string;
}

/**
 * 图标组件
 * 
 * 方便字符串icon转为 ant icon 或其它
 */
export default function Icon({
  icon,
  color,
}: IconProps): JSX.Element {
  let iconWidget: React.ReactNode = <BulbOutlined/>;

  if (typeof antIcons?.[icon] !== 'undefined') {
    iconWidget = antIcons?.[icon];
  }

  return (
    <Container
      className={cls(`${CLASS_PREFIX}-icon`)}
      color={color}
    >
      {iconWidget}
    </Container>
  );
}
