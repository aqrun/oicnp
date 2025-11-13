---
title: '第3章 电商网站前端开发'
description: '如何架构前端应用 如何取得数据 如何显示数据 如何修改数据'
reading_time: 3

taxonomies:
  categories: ['reading', 'graphql-in-action']
  tags: ['graphql', 'restful', '全栈']
---

本单主要解决的问题：

* 如何架构前端应用
* 如何取得数据
* 如何显示数据
* 如何修改数据

了解 GraphQL API 设计的奥义：

* 基本思路1：基于 Schema，尽量少甚至不关心后端具体实现
* 基本思路2：前端代码要易维护、易修改、易测试
* 基本思路3：尽量减少代码重复，做到 DRY

## 3.1 GraphQL 前端开发要点

GraphQL 的前端开发主要是针对前端技术和 GraphQL 客户端的结合使用。

### 3.1.1 前端开发的主要任务

#### 1. 构建查询

使用 Schema 根据业务逻辑按需构建 Query / Mutation / Subscription

#### 2. 拿到数据

发送查询然后解析返回结果，配合缓存减少重复请求

#### 3. 显示数据

返回的 JSON 数据映射到UI元素

#### 4. 和数据互动

部分互动需要发送数据到服务器（如回复贴子）

### 3.1.2 前端开发的难点

#### 1.保证数据一致性

从服务器拿到的数据有多种存储位置（UI显示、客户端缓存、服务器缓存）

#### 2.减少重复代码

保证代码 DRY、易读性、可维护性

#### 3.结合使用前端框架

### 3.1.3 前端技术的选型

使用 Apollo GraphQL 客户端结合 React

## 3.2 前端 React 项目初始化

### 3.2.1 React 特点简介

React 最大的特点是采用声明式的方式构建用户界面，强调用户界面自动与数据保护同步

### 3.2.2 React 整合 GraphQL 前端系统设计

![React to GraphQL](https://cdn.oicnp.com/images/2024/react-to-graphql.png)

根节点使用 `ApolloProvider` 包起来，需要整合 GraphQL 数据的组件使用 `graphql()` 函数包起来。

* 项目结构保持不变
* 只有一部分组件需要结合 GraphQL

### 3.2.3 创建 React 前端工程

参考 nextjs

### 3.2.4 安装 Appollo 客户端

`yarn add react-apollo`

Appolo 的 Graphql 客户端除了负责网络数据传输，还承担：

* 为服务端数据提供本地缓存
* 自动连接 React 组件和 GraphQL
* 分页处理
* 支持基于 websocket 的数据订阅，获取实时数据

### 3.2.5 初始化 Graphql 客户端

```ts
import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

import { ApolloProvider } from 'react-apollo';
import { ApolloClient } from 'apollo-client';
import { HttpLink } from 'apollo-link-http';
import { InMemoryCache } from 'apollo-cache-inmemory';

const graphQLServerLink = new HttpLink({
  // GraphQL 服务端地址 URI
  uri: 'http://localhost:8888/graphql',
});

// 创建一个 GraphQL 客户端
const client = new ApolloClient({
  link: graphQLServerLink,
  cache: new InMemoryCache(),
});

// 用 ApolloProvider 包住 React 前端应用的根节点 App
// 并把前面创建好的 client 传给 apolloProvider
const graphQLApp = (
  <ApolloProvider client={client}>
    <App />
  </ApolloProvider>
);

ReactDOM.render(graphQLApp, document.getElementById('root'));
```

### 3.2.6 手动发送查询

只发送请求，无UI显示可以直接使用 ApolloConsumer 组件，调用 client.query() 方法。

## 3.3 只读数据的 React UI 组件

需求 实现一个产品列表的 React UI 组件，只需要显示产品的ID和name，并把它放到建立的项目中。

### 3.3.1 构建 GraphQL Query 查询

```ts
import gql from 'graphql-tag';

const ProductListQuery = gql`
  query {
    allProducts {
      id
      name
    }
  }
`
```

gql 函数把 graphql 的查询字符串解析成一个抽象语法树

### 3.3.2 定义列表元素组件

先定义一个产品条目的 React UI 组件

```tsx
export interface ProductItemProps {
  // 产品数据
  product: Product;
}

export const ProductItem: React.FC<ProductItemProps> = ({
  product,
}) => {
  return (
    <a className="panel-block">
      {product.name}
      ({product.id})
    </a>
  );
};
```

### 3.3.3 定义列表组件

```tsx
export interface ProductListProps {
  loading?: boolean;
  allProducts?: Product[];
}

export const ProductList: React.FC<ProductListProps> = ({
  loading,
  allProducts,
}) => {
  if (loading) {
    return (
      <div>
        Loading
      </div>
    );
  }

  return (
    <div className="panel">
      <p className="panel-heading">
        Product List
      </p>
      {allProducts?.map((item) => {
        <ProductItem
          key={item?.id}
          product={item}
        />
      })}
    </div>
  );
}
```

### 3.3.4 绑定静态查询和UI组件

```ts
import { graphql } from 'react-apollo';

// 查询和 UI 组件绑定到一起
const ProductListWithData = graphql(ProductListQuery)(ProductList);

export default ProductListWithData;
```

### 3.3.5 使用Query组件

Apollo 提供了 Query 组件以简化GraphQL和React组件的绑定。

```ts
import { Query } from 'react-apollo';

export const ProductList = () => {
  return (
    // 给 Query 组件传入提前定义好的查询
    <Query query={ProductListQuery}>
      {({ loading, error, data }) => {
        if (loading) {
          return (<div>Loading</div>);
        }

        return (
          <div className="panel">
            <p className="panel-heading">
              Product List
            </p>
            {data.allProducts.map((product) => {
              return (
                <ProductItem
                  key={product.id}
                  product={product}
                />
              );
            })}
          </div>
        );
      }}
    </Query>
  );
};
```

主要分三步：

* 1.告诉Query组件要进行什么样的查询
* 2.定义 Query 组件的render函数并接受三个参数 loading/error/data
* 3.在 render 函数中定义如何处理 loading 和 error 状态及如何生成视图

### 3.3.6 从Query组件接收一个参数需求，为每个商品制作一个展示页面。点击商品列表显示对应的商品展示页，只展示一种商品，用户可以查看更多该商品细节。

多页面需要增加 router 支持

```shell
yarn add react-router-dom
```

```ts
<Switch>
  <Route exact path="/" component={ProductList} />
  <Route path="/product/:id" component={ProductDetail} />
</Switch>
```

详情查询

```ts
const ProductDetailQuery = gql`
  query($productId: ID!) {
    product(id: &productId) {
      id
      name
      inStock
      price
      isFreeShipping
    }
  }
`;
```

Query 组件中可以使用 variables 属性指定变量对应查询定义的变量

```ts
const ProductDetail = (props) => {
  <Query
    query={ProductDetailQuery}
    variables={{
      // 这个商品ID来自 react-router 对URL的解析--如 product/333这里的值就是333
      productId: props.match.params.id,
    }}
  >
    {({ loading, error, data}) => {
      // ...
    }}
  </Query>
};
```

### 3.3.7 数据的接收及出错处理

GraphQL 查询有三种可能的状态：

* 下载中 - loading
* 已出错 - error
* 数据准备好了 - data

对应 Query组件内层函数的三个参数

### 3.3.8 手动刷新

刷新产品列表在Query组件使用refetch函数

```ts
data.refetch(variables);
```

使用不同的变量可以实现类似翻页之类的功能

## 3.4 修改数据的 React UI 组件

对于数据一致性要求比较高的应用，一定要以服务器端返回的数据为准。

需求：下单，在商品展示页面，提供一个简单表单，包括购买数量的输入框和提交按钮。用户点击提交后，发送
GraphQL请求到服务器端。服务器返回成功后，利用服务器端返回的数据，更新当前页面库存。

### 3.4.1 定义一个带有变量的 Mutation 操作

```js
const MakeOrderQuery = gql`
  mutation makeOrder($productID: ID!, $quantity: Int) {
    makeOrder(productID: $productID, quantity: $quantity) {
      id
      inStock
    }
  }
`;
```

### 3.4.2 使用 Mutation UI 组件

扩展 ProductDetail 组件

```tsx
const ProductDetail = (props) => {
  let quantityInput;

  return (
    <div>
      <Query ...>
        ...
      </Query>
      <Mutation mutation={MakeOrderQuery}>
        {(makeOrder, { data }) => {
          <div>
            <input
              defaultValue="0"
              ref={node => {
                quantityInput = node;
              }}
            />
            <button
              onClick={() => {
                makeOrder({
                  variables: {
                    quantity: quantityInput.value,
                    productId: props.match.params.id
                  }
                })
              }}
            >
              MakeOrder
            </button>
          </div>
        }}
      </Mutation>
    </div>
  );
};
```

使用 Mutation 组件的主要步骤是：

1. 绑定 Mutation 查询： `<Mutation mutation={MakeOrderQuery} />`。
2. 在 Mutation 组件的 children 里定义函数。
3. 在某个 UI input 组件的事件处理器中调用render函数传入进来的回调函数。

## 3.5 支持订阅

订阅（Subscription）是一种服务器端向客户端推送数据的工作方式。和Query相比，订阅不是拿到实时的数据，
而是注册若干事件的监听器。

### 3.5.1 什么时候使用订阅

* 更新延时要求特别高，如需要一秒响应
* 某些应用客户端在得到初始数据后，会频繁地发生很多小修改。如在线文档编辑

### 3.5.2 订阅是如何实现的

一般采用 web socket 实现。需要在客户端和服务器端同时支持 WebSocket.

参考开源项目： subscriptions-transport-ws

## 3.6 本地数据

本书关注点在于把本地数据当作服务器数据的本地副本或者说是缓存来使用，难点在于如何保证数据一致性。

Q&A 购物车数据是放在本地好还是服务器端好？

都可以接受。建议放在服务器端。


