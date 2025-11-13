---
title: Less module Global应用
description: '前端Less module 部分global特殊写法'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'less', 'global']
---

## 正常情况

```less
.some-container {
  :global {
    .some-child {
      color: red;
    }
  }
}
```

## 标签动态增加 loading class

`<div class="some-container loading"></div>`

```less
.some-container {
  &:global(.loading) {
    .noDataTitle {
      color: red;
    }
  }
}
```

## 移动端适配媒体查询

```less
.some-container {
  width: 100%;

  // 1000 以上度改为50%
  @media screen and (min-width: 1000px) {
    width: 50%;

    .some-child {
      margin-right: 16px;
    }
  }
}
```

## 媒体查询配合伪类

```less
.some-container {
  width: 100%;

  @media screen and (min-width: 1000px) {
    width: 50%;

    // 第偶数个子项margin-right为0
    &:global(:nth-child(2n)) {
      .some-child {
        margin-right: 0;
      }
    }
  }
}
```
