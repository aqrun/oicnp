---
title: '架构模式之单一数据源SSOT'
description: '单一数据源架构模式的一种， 类比MVC、MVVM、MVP、MVI、Compose。所有进入系统的入口点(REST、GraphQL和RPC)都将使用相同的验证、授权和错误处理规则'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'graphql', 'auth', 'ssot']
---

单一数据源(SSOT - Single source of truth) ，架构模式的一种， 类比 MVC、MVVM、MVP、MVI、Compose

## Redux 三原则：

Redux 是 Web 端的状态管理库，深受 Flux、CQRS、Event Sourcing 的影响。Redux 三原则包括：

### 1. Single Source of Truth

单一数据源，App 的全局状态存储在一个 tree 对象里面，而这个 tree 对象被单一的 store 持有。

### 2. State is read-only

状态只读。

### 3. Changes are make with pure functions

状态的改变动作必需是纯函数。

## Graphql 业务逻辑层

实际的业务逻辑、数据验证、权限认证都是在业务逻辑层实现。Graphql 建议业务逻辑层要遵循业务域的单一数据源规则。

![Graphsql Business layer](https://static.oicnp.com/blog/2023/business_layer.png)

如图，所有进入系统的入口点(REST、GraphQL 和 RPC)都将使用相同的验证、授权和错误处理规则进行处理。

## 参考链接

- [Thinking in Graphs](https://graphql.org/learn/thinking-in-graphs/#business-logic-layer)
- [架构设计之 Single source of truth](https://blog.zhangwen.site/single-source-of-truth/)
