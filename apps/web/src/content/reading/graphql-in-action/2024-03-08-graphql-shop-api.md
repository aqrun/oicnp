---
title: '第2章 GraphQL 初体验 电商API设计'
description: 'GraphQL 是什么？能解决什么问题？有什么优势和缺点？好的开源实现？'
reading_time: 3

taxonomies:
  categories: ['reading', 'graphql-in-action']
  tags: ['graphql', 'restful', '全栈']
---

解决的问题：

* 如何使用 GraphQL 设计 API？
* 如何使用 GraphQL 和服务端互动（查询语法）？
* GraphQL 好用的语法？

需求：提供一个电商服务 API，核心功能是可以查询多种商品信息。

要解决的问题：

* 定义和描述数据
  * 数据类型
  * 数据结构
* 定义和描述数据的操作方式
  * 如何查询数据（读）
  * 如何修改数据（写）

先有定义API的 Schema 前后端开发可以分别进行，无须相互依赖。

## 2.1 基本开发环境的搭建

本例使用 Node.js 和 Express-GraphQL 快速搭建开发环境。

```bash
git clone https://githbu.com/beinan/graphql_server_starter.git
# 检出所有分支
git fetch
# 空白分支
git checkout i_am_a_beginner
# 成型的迷你电商后端
git checkout mini_store
# 安装依赖  or yarn install
npm install
# 启动服务端
npm run
```

## 2.2 和 GraphQL 互动

### 2.2.1 交互界面 GqphiQL 的使用

GraphiQL 是一个基于浏览器的 GraphQL 集成开发测试环境。

```
http://localhost:8888/graphiql
```

![GraphiQL](https://github.com/graphql/graphiql/blob/main/packages/graphiql/resources/graphiql.png?raw=true)

### 2.2.2 使用 curl 发送请求

不方便使用浏览器的开发环境可以使用 curl 来发送请求：

```bash
curl -X POST -H "Content-Type: application/json" \
  --data '{ "query": "{ \
    getUser(id: \"beinan\") { id name } \
  }" }' \
  http://localhost:8888/query
```

### 2.2.3 使用第三方客户端

* Altair

## 2.3 Schema 与定义数据类型

GqphQL 是服务端和客户端沟通的一个“合同”

### 2.3.1 强类型的查询语言

GraphQL 是强类型的查询语言，基传入传出的数据都需要有与之对应的类型。

GraphQL 的类型系统是在运行时，也就是数据传入传出过程中动态来帮助检测数据合法性的。

### 2.3.2 服务器端的 Schema

所有数据类型和查询都要在这个事先定义的 Schema 中有据可查。

数据查询中，使用预先定义的 Schema 有什么好处？

* 更好的保证数据正确
* 前后端可依据 Schema 分别开发

### 2.3.3 标量类型

* Int 整形
* Float 浮点型
* String 字符串
* Boolean 布尔型
* ID 标识符型
* 自定义标量

```js
scalar Date
scalar Email
```

自定义标量是从语义的层面设计的，数据在实际传输过程中是普通的字符串。

### 2.3.4 自定义复杂类型

需求 商品中包括id、名称、价格、库存和是否包邮5个信息。

```ts
type Product {
  id: ID,
  name: String,
  price: Float,
  inStock: Int,
  isFreeShipping: Boolean
}
```

### 2.3.5 枚举

有少量的可选项，且只能选择一种，可以使用枚举类型，如性别。

需求：设计一个用户类型，提供性别字段，性别有男、女、其他、未知四种:

```ts
enum Gender {
  Male,
  Female,
  Other,
  Unknown
}

type User {
  id ID!
  name String,
  gender Gender
}
```

### 2.3.6 列表以及对象的列表

需求 希望用户可以支持多个昵称：

```ts
type User {
  id ID!
  name: String
  nickname: [String]
  gender: Gender
}
```

需求 每个用户可以有自已的商品收藏夹

```ts
type User {
  id ID
  name String
  nickname: [String]
  gender Gender
  favorites: [Product]
}
```

## 2.4 定义操作

### 2.4.1 只读查询操作

需求 为 API 提供两个查询操作，一个获得所有产品，一个根据 ID 查询具体的产品。

```ts
type Query {
  allProducts: [Product]
  product(id: ID!): Product
}

schema {
  query: Query
}
```

示例：拿到所有产品的ID和name

```ts
query {
  allProducts {
    id
    name
  }
}
```

返回数据：

```json
{
  "data": {
    "allProducts": [
      {
        "id": "10001",
        "name": "iPhone X"
      },
      {
        "id": "1000",
        "name": "A"
      }
    ]
  }
}
```

GraphQL 查询的两个优点：

* 高效： 只会返回客户端实际需要的字段
* 同构： 客户端发出的查询和服务器返回的结果结构相同

### 2.4.2 可写修改操作

需求：为迷你电商网站建立一个“下单”功能，下单操作返回新订单的数据

```ts
type Mutation {
  makeOrder(productId: ID!, quantity: Int): Product
}

schema {
  query: Query
  mutation: Mutation
}
```

示例：对 “10001” 号产品下单，并返回新订单指定字段：

```ts
mutation {
  makeOrder(productId: "10001", quantity: 2) {
    id
    name
    inStock
  }
}
```

返回数据：

```ts
{
  "data": {
    "makeOrder": {
      "id": "1001",
      "name": "iPhone",
      "inStock": 58
    }
  }
}
```

### 2.4.3 订单操作

订阅是服务器端主动推送数据给客户端，而查询是客户端主动从服务器端读取数据。

功能实现需要使用 web-socket 或 http2 支持持久链接的协议来实现服务器端的推送。

```ts
type Subscription {
  newProduct(): Product
}
```

### 2.4.4 传递输入类型

传入数据属于复杂结构的数据时需要定义一个输入类型(Input type)。

*需求* 扩展下单操作，使其可以支持一次购买多个商品，并在订单中加入收货地址。

```ts
input OrderItemInput {
  productID: ID!,
  quantity: Int
}

input OrderInput {
  items: [OrderItemInput]
  address: String
}

type mutation {
  makeOrder(productID: ID!, quantity: Int): Product
  makeOrderV2(order: OrderInput): [Product]
}
```

请求示例：

```ts
mutation {
  makeOrderV2 (order: {
    items: [
      {
        productID: "1001",
        quantity: 3,
      },
      {
        productID: "1002",
        quantity: 4,
        address: "LA, CA",
      }
    ]
  }) {
    id
    name
    inStock
  }
}
```

### 2.4.5 操作也是字段

## 2.5 精炼数据模型与操作

Schema 定义出现冗余时如何优化精炼

### 2.5.1 接口和继承

*需求* “诗酒趁年华” —— 我们网站同时支持两种特定商品：红酒和图书。红酒一定要有一个年份的字段，页图书一定要有一个书号（ISBN）字段。

interface 表达 Product 抽象类。

```ts
interface Product {
  id: ID!,
  name: String!,
  price: Float,
  inStock: Int,
  isFreeShipping: Boolean,
  images: [String]
}
```

接口是抽象数据类型不可以直接为抽象数据类型创建实现

```ts
type Wine implements Product {
  id: ID!,
  name: String!,
  inStock: Int,
  isFreeShipping: Boolean,
  images: [String],
  year: Int!
}

type Book impltents Product {
  id: ID!,
  name: String!,
  inStock: Int,
  isFreeShipping: Boolean,
  images: [String],
  isbn: String!,
}
```

GraphQL 中子类必须重载接口里所有的字段。

子类中的字段的类型可以和接口中的同名字段的类型不同，但必须是接口中同名字段类型的子类或非空类型。

interface Product 中增加字段：

```ts
releatedProduct: [Product]
```

在子类 Wine 中可以覆盖 releatedProduct 这个字段的类型：

```ts
relatedProduct: [Wine]
```

有了多态和继承，只需定义一套操作就可以同时覆盖两种商品：

```ts
type Query {
  allProducts(pageNum: Int = 10, pageSize: Int = 20): [Product]
  product(id: ID!): Product
}
```

避免过度使用复杂的继承结构，减少项目维护的困难。

### 2.5.2 联合

*需求* 书和新朋友 —— API 提供一个搜索功能，返回的结果里可以有书，也可以有新朋友。

接口和继承适合有公共字段的类型，没有公共字段的几种类型放在一起查询可以使用联合(Union)

```ts
union Resource = Book | User
```

不能以接口或联合为成员来创建一个联合，如 `union Resource = Product | User`, Product 是接口，这就是一个非法定义。

## 2.6 精炼查询

如何合并查询中重复的部分

### 2.6.1 使用变量

```ts
query ($productID: ID!) { // 声明变量，需要提供变量名和类型
  getProduct(id: $productID) { // 在需要使用变量的地方提供变量名
    id
    name
  }
}
```

指定默认值：

```ts
query ($productID: ID = "1001") {
  getProduct(id: $productID) {
    id
    name
  }
}
```

### 2.6.2 使用别名

*需求* 做一个商品比较查询，根据客户需要一次返回两个商品信息。

> 提示：调用两次 product 查询操作。

```ts
query {
  // prod1 和 prod2 就是两个别名
  prod1: product (id: "1001") {
    id
    name
  }
  prod2: product(id: "1002") {
    id
    name
  }
}
```

返回结果：

```json
{
  "data": {
    "prod1": {
      "id": "1001",
      "name": "iPhone x"
    },
    "prod2": {
      "id": "1002",
      "name": "A Brief History of Time"
    }
  }
}
```

### 2.6.3 使用片段

```ts
query {
  prod1: product (id: "1001") {
    ...prodFields
    inStock  // 使用片段时可以附加额外字段 InStock
  }
  prod2: product(id: "1002") {
    ...prodFields
  }
}

// 定义片段
fragment prodFields on Product {
  id
  name
}
```

片段 fragment 必须依托某个数据类型。

### 2.6.4 类型条件

*需求* 构建一个查询，查找书和新朋友，如果是书需要书的 id 和书号 isbn 两个字段；如果是用户需要 id 和其父亲名字 father{name} 两个字段

```ts
query {
  allResource() {
    id                            // 共有字段
    ... on User { father {name} } // 如果是用户
    ... on Book { isbn }          // 如果是图书
  }
}
```

GraphQL 标准中使用 `... on TypeName { field1, field2 .. }` 这样的表达称为内联片段。

### 2.6.5 使用 Directive

*需求* 某个移动应用需要根据目前屏幕的大小来决定获取内容的多少，如，如果是窄屏手机，就不显示产品图片了，只显示名称。原来的实现是通过写两个不同的查询解决的，但现在只想用一个查询达到目的。

之前的查询：

```ts
query forNarrowScreen {
  product(id: "1001") {
    id
    name
  }
}

query forBigScreen {
  product(id: "1001") {
    id
    image
  }
}
```

优化为一个查询：

```ts
query forAllScreen ($isNarrowScreen: Boolean) {
  product(id: "0001") {
    id
    name @include(if: $isNarrowScreen)
    image @skip(if: $isNarrowScreen)
  }
}
```

通过在字段后指定两种 Directive 的方式来决定字段的去留。

* `@include(if: $isNarrowScreen)` if 后面表达式为真就保留该字段
* `@skip(if: $isNarrowScreen)` 和 @include 相反为真则剔除该字段

### 2.6.6 后端工程师的福音

变量、别名、片段 和 Directive 只在客户端有意义，对服务器端 Schema 来说是透明的。

## 2.7 简单的数据验证

GraphQL 对客户端发送的查询请求和服务器端返回的数据结果响应都会进行验证。多个查询操作任意一个验证不通过，所有操作都不会被执行。

### 2.7.1 必填填的验证

```ts
type Product {
  id: ID!,        // 非空字段
  name: String!,  // 非空字段
  price: Float,
  inStock: Int,
  isFreeShipping: Boolean,
  images: [String]
}
```

### 2.7.2 标量值的验证

Scalar 定义的自定义标量类型，需要手动实现自定义验证规则。

