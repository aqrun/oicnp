---
title: '第 1 章 GrphQL API 设计和全栈开发'
description: 'GraphQL 是什么？能解决什么问题？有什么优势和缺点？好的开源实现？'
reading_time: 3

taxonomies:
  categories: ['reading', 'graphql-in-action']
  tags: ['graphql', 'restful', '全栈']
---

主要解决的问题：

- GraphQL 是什么？它能解决什么问题？
- GraphQL 有什么优势和缺点？
- GraphQL 有哪些好的开源实现？

## 1.1 什么是 GraphQL

Facebook 2015 年正式发布的一种全新数据查询方式，优雅地解决了客户端与服务器端数据交换难题。

## 1.2 分布式系统

多台计算机通过计算检网络协同在一起为客户服务的工作方式，称为分布式系统。

一个篱笆三个桩，一个好汉三个帮

分布式系统和 GraphQL 相关的特点：

- 扩展性
- 可靠性
- 远程资源共享
- 更强的处理能力

## 1.3 C/S 架构与 API

### 1.3.1 C/S 架构

在设计分布式系统时，把功能单元分成两种角色 —— 服务器和客户端。提供资源的一方是服务器，请求资源的一方是客户端，这种设计方式也就是 C/S 架构方式。

### 1.3.2 前端和后端

程序员根据技术方式分为前端工程师和后端工程师。

### 1.3.3 全栈程序员

部分程序员兼顾前端与后端。

全栈程序员的特点：

- 懂后端
- 懂前端
- 懂设计 API
- 懂质量保障
- 懂信息安全

### 1.3.4 应用程序接口

为了便于服务器和客户端之间、层与层之间、模块与模块之间互相协作，需要定义一套清晰明确的接口来让它们互相调用。这套接口就称作应用程序接口（Application Programming Interface， 简称 API）

## 1.4 RESTful API 的起源和特点

### 1.4.1 仓库保管员的窘境

仓库保管员小刚日常处理上千人次物品领取工作

表述性状态迁移，也就是 REST（Representational State Transfer），基于这种方式设计出来的 API，称作 RESTful API, 而提供 RESTful API 的服务，一般称为 RESTful 服务。

### 1.4.2 REST 无状态的好处

- 降低了服务器处理问题的难度
- 降低响应延时
- 提高系统可靠性（Reliability）、可用性(Availability)、弹性(Resilience)

### 1.4.3 RESTful API 是否真的无状态

服务器不保存用户会话状态，并不是客户端无状态，状态保存在了客户端。

### 1.4.4 RESTful API 是否是数据传输协议

并没有形成一种数据传输协议，而是诞生了一种系统设计的模式。

### 1.4.5 RESTful API 的好处

- 简单直接
- 扩展性强
- 兼容性强

## 1.5 RESTful API 的主要问题

- 数据定制问题
- 多次请求的问题
- 异常处理的问题
- 返回数据格式未知问题
- 请求 Endpoint 和方式过多的问题

## 1.6 GraphQL 如何解决 RESTful API 的问题

### 1.6.1 GraphQL 中可以自由定制数据

```json
{
  user() {
    name
    age
  }
}
```

限定返回结果的字段，只查询 name 和 age

### 1.6.2 可以合并多次请求

```json
{
  user() {
    name
    age
  }
  product() {
    id
    name
  }
}
```

请求用户的同时可以请求产品

### 1.6.3 GraphQL 错误以及异常信息明确

多个查询合并时，可能有的查询会失败，出错信息需要明确清晰。

```json
{
  "data": {
    "user": {
      "id": "9527",
      ...
    },
    "product": null, // 出错返回 null 代表不存在
  },
  "errors": [
    {
      // message 承载了出错信息
      "message": "Not found",
      // path 指示哪个查询出错
      "path": [
        "product"
      ]
    }
  ]
}
```

### 1.6.4 GraphQL 返回数据的形式和查询请求同构

调用方可以明确返回数据的结构。

### 1.6.5 GraphQL 使用单一的 Endpoint

在查询中定义需要的数据，各种查询拼装到一起发送到唯一的 Endpoint

### 1.6.6 GraphQL 替代了什么

借助 GraphQL 的优点，让开发者可以尽情地设计所需的数据结构，更多关注数据实体之间的关系，可以各种数据上自由地查询和组合，API 的设计可以真正的推脱视图的束缚。

### 1.7 GraphQL 引发的疑虑

常见问题

- GraphQL 是否还是 RESTful
- 增大了后端系统设计难度
- 是否会带来后端性能问题
- 迁移到 GraphQL 的代价
  - 不需要换语言
  - 不需要换框架
  - 可以和 RESTful 共存
  - 可以和微服务集群共存
  - 需要考虑后端优化
- GraphQL 最好是全栈工程师驱动

### 1.8 GraphQL 全线框架的选用

#### Relay

Facebook 出品，GraphQL/React 亲兄弟，功能强大。

#### Apollo

开源社区的 GraphQL 全栈解决方案

