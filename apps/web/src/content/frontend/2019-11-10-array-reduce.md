---
title: '数组常用操作 reduce'
description: '数组操作'
slug: js-array-reduce

taxonomies:
  categories: ['frontend', 'article']
  tags: ['array', 'reduce']
---

```js
const products = [
  { id: 1, name: 'alex', price: 10 },
  { id: 2, name: 'bob', price: 20 },
  { id: 3, name: 'haha', price: 50 },
];

const carts = [{ id: 1, quantity: 2 }];

// 合并
const list = carts.map((item) => {
  // 查找一个
  const p = products.find((i) => i.id === item.id);
  return { ...item, ...p };
});
console.log('list', list);

// 计算总计
const total = list.reduce((total, product) => {
  return total + product.price * product.quantity;
}, 0);
console.log('total', total);
```
