import {} from '@ant-design/icons';
import { CLASS_PREFIX } from '~/constants';
import cls from 'clsx';
import {
  LaptopOutlined,
  UserOutlined,
} from '@ant-design/icons';
import { Container } from './index.styled';

export interface IconProps {
  icon: string;
}

/**
 * 图标组件
 * 
 * 方便字符串icon转为 ant icon 或其它
 */
export default function Icon({
  icon,
}: IconProps): JSX.Element {
  let iconWidget: React.ReactNode = '';

  switch (icon) {
    case 'user-outlined':
      iconWidget = <UserOutlined />;
      break;
    case 'laptop-outlined':
      iconWidget = <LaptopOutlined />;
      break;
    default:
      iconWidget = <UserOutlined />;
  }

  return (
    <Container className={cls(`${CLASS_PREFIX}-icon`)}>
      {iconWidget}
    </Container>
  );
}
