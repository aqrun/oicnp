import Link from 'next/link';
import { Menu } from 'antd';
import { mainMenu } from '../../constants';
import {
  Container,
} from './index.styled';

export const LayoutHeader = () => {

  return (
    <Container className="header">
      <div className="logo">
        <Link href="/">
          <img src="/icons/logo.svg" />
        </Link>
      </div>
      
      <Menu theme="dark" mode="horizontal" defaultSelectedKeys={['2']}>
        {mainMenu.map((item) => {
          return (
            <Menu.Item
              key={item.id}
            >
              <Link href={item.href}>{item.name}</Link>
            </Menu.Item>
          );
        })}
      </Menu>
    </Container>
  );
}