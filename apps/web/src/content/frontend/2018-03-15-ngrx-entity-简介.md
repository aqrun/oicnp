---
title: '@ngrx/entity 简介'
description: '@ngrx/entity，是NgRx团队中第一个旨在减少样板的类库。 其目标是帮助开发人员编写维护实体集合的Reducer功能。'
slug: ngrx-entity

taxonomies:
  categories: ['frontend', 'article']
  tags: ['angular', 'ngrx', 'ngrx/entity']
---

> [原文链接：https://medium.com/ngrx/introducing-ngrx-entity-598176456e15](https://medium.com/ngrx/introducing-ngrx-entity-598176456e15)

关于使用 NgRx 构建应用程序最常见的抱怨之一是开发人员必须编写的样板代码数量。 试图解决样板问题时的挑战在于，为了从体系结构中获得全部优势，必需的明确性至关重要。 我们认为，通过开发有针对性的库，我们可以帮助减少一些这种样板，同时保留 NgRx 提供的所有好处。

今天，我们很高兴宣布发布@ngrx/entity，它是 NgRx 团队中第一个旨在减少样板的类库。 其目标是帮助开发人员编写维护实体集合的 Reducer 功能。

## 她是如何工作的

@ngrx/entity 允许您为不同类型的实体创建实体适配器(entity adapter)。 使用实体适配器，您可以快速编写 reducer 操作并自动生成选择器(selector)。 例如我们的 [example application](https://ngrx.github.io/platform/example-app)，假设我们想编写一个管理书籍集合的应用程序。 Book 接口(interface)如下所示：

```ts
interface Book {
  id: string;
  title: string;
}
```

编写管理这个集合的 reducer 的第一步是创建一个实体适配器(entity adapter)：

```ts
import { createEntityAdapter } from '@ngrx/entity';

const bookAdapter = createEntityAdapter<Book>();
```

接下来，我们需要为我们的书籍状态声明接口：

```ts
import { EntityState } from '@ngrx/entity';

export interface BookState extends EntityState<Book> {}
```

EntityState 的声明如下所示：

```ts
interface EntityState<V> {
  ids: string[];
  entities: { [id: string]: V };
}
```

我们维护一个 ID 列表和 Entity 字典的主要原因有两点：

1. 我们希望快速查找特定的实体。 如果您只想从 Store 中选择一本书，使用 Entity 词典比搜索数组要快得多
2. 我们也想维护列表的顺序。 如果你想保持列表排序，这是特别重要的！

`EntityState<V>`的声明符合所有目标。 它也是可扩展的，所以我们可以在书籍集合中包含其他相关信息，例如当前选择的书籍。

接下来，定义一些操作：

```ts
improt { Action } from '@ngrx/store';

export enum BookActionTypes {
    ADD_ONE = '[Books] Add One',
    UPDATE_ONE = '[Books] Update One',
    DELETE_ONE = '[Books] Delete One',
    GET_ALL = '[Books] Get All',
}

export class AddOne implements Action {
    readonly type = BookActionType.ADD_ONE;

    constructor(public book: BookModel) {}
}

export class UpdateOne implements Action {
    readonly type = BookActionType.UPDATE_ONE;
    constructor(
        public id: string,
        public changes: Partial<BookModel>
    ) {}
}

export class DeleteOne implements Action {
    readonly type = BookActionType.DELETE_ONE;
    constructor(
        public id: string
    ) {}
}

export class GetAll implements Action {
    readonly type = BookActionType.GET_ALL;
    constructor(
        public books: BookModel[]
    ) {}
}

export type BookActions = AddOne
    | UpdateOne
    | DeleteOne
    | GetAll
    ;

```

现在我们准备使用 bookAdapter 来创建我们的书籍 reducer：

```ts
const initialState: BookState = bookAdapter.getInitialState();

export function bookReducer(
  state: BookState = initialState,
  action: BookActions
) {
  switch (action.type) {
    case BookActionTypes.ADD_ONE:
      return bookAdapter.addOne(action.type, state);
    case BookActionTypes.UPDATE_ONE:
      return bookAdapter.updateOne(
        {
          id: action.id,
          changes: action.changes,
        },
        state
      );
    case BookActionTypes.DELETE_ONE:
      return bookAdapter.deleteOne(action.id, state);
    case BookActionTypes.GET_ALL:
      return bookAdapter.addAll(action.books, state);
    default:
      return state;
  }
}
```

新的状态可以使用新创建的 reducer 在 Store 中注册。 我们需要做的最后一件事就是生成用于处理这个状态的选择器(selector)：

```ts
export const { selectIds, selectEntities, selectAll, selectToAll } =
  bookAdapter.getSelectors();
```

那么样板代码做了哪些？

1. 不再需要显式声明状态接口(state interface)的所有属性
2. 添加，删除或更新状态实体(state entity)的实现都由适配器处理。
3. 该适配器会为您生成一组常用的选择器。

关于 Entity 的更多文档查看 [NgRx Gihub 仓库](https://github.com/ngrx/platform/tree/master/docs/entity) 以及 [example application](https://github.com/ngrx/platform/tree/master/example-app)示例项目中的实际应用。

## NgRx 4.1 发布

除了发布 @ngrx/entity 之外，我们还发布了 Store，Effects 和 Router Store 的 v4.1.0，其中包含许多新功能和错误修复。 有关包含内容的更多信息，[请查看更新日志](https://github.com/ngrx/platform/blob/master/CHANGELOG.md)。

## 帮助支持 NgRx

随着 NgRx4，我们宣布创建我们的 [OpenCollective](https://opencollective.com/ngrx)，您可以帮助支持 NgRx 的开发。 我们非常感谢来自支持者、赞助商和社区的支持。 作为这些贡献的结果，Entity 是第一个创建的库。 Entity 是为 NgRx 平台创建更多库的第一步。 考虑[通过 OpenCollective](https://opencollective.com/ngrx)支持这些开发工作。
