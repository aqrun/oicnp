---
title: 'NgRx Store 理解状态选择器(state selectors)'
description: '选择器是纯函数，它将状态切片作为参数，并返回可传递给组件的一些状态数据，介绍如何定义`createFeatureSelector`、`createSelector`, 以及它们的区别'
slug: ngrx-store-selector

taxonomies:
  categories: ['frontend', 'article']
  tags: ['angular', 'selectors', 'ngrx', 'state']
---

> [原文链接：https://toddmotto.com/ngrx-store-understanding-state-selectors](https://toddmotto.com/ngrx-store-understanding-state-selectors)

选择器是纯函数，它将状态切片作为参数，并返回可传递给组件的一些状态数据。 为了更好地理解选择器是什么以及它们做什么，它有助于将 ngrx 状态看作数据结构 —— 一种可以序列化为 JSON 的树。 数据通过在 reducer 中组成状态添加到状态树中 - 这是最简单的部分。 现在为了从状态树中获取数据，我们必须遍历它来找到我们感兴趣的属性并返回它。 这可能会变得更加复杂，这也是选择器帮助我们的地方。

您可能已经看到正在使用 store.select 方法通过传递字符串值来从 store 获取数据：

```ts
this.store.select('pizzas');
```

该字符串代表 store 中状态切片的名称，我们可以预料此函数会返回与我们的 pizzas 属性相对应的数据 - 可能是比萨饼数组。 但是，`store.select` 也可以传递一个函数，这个函数参数是一个状态切片返回状态的一个属性（您可能已经看到过）：

```ts
this.store.select((state) => state.pizzas);
```

这两种方法都代表了选择器的概念 - 我们正在“选择”状态！

所以，当我们将 ngrx/store 作为一个数据库，而选择器就像 SQL 查询中的 SELECT 一样 - 它们给我们提供了我们想要的信息。 随着我们的状态树越来越深入，将状态从 Store 中获取数据变得越来越复杂。

我们可能会发现自己在组件中编写复杂的数据转换逻辑，以获得我们需要的东西 - 但我们不想这么做 - 而这正是选择器的作用。 就像数据库一样，我们可以通过组合选择器来完成数据转换，只需返回我们需要的数据。 我们将保持我们的组件简洁和从 store 解耦。

## 目录

- 考虑数据结构
- 特征状态选择器(Feature state selector)
- 状态切片选择器(State slice selector)
- 总结

## 考虑数据结构

首先让我们将这种状态概念可视化为 NGRX 上下文无关的数据结构。 我们创建一个具有 state 属性和一些初始值的 JavaScript 类 Store：

```ts
class Store {
  constructor() {
    this.state = {
      products: {
        pizzas: {
          entities: {
            1: { name: 'Pizza 1', id: 1 },
            2: { name: 'Pizza 2', id: 2 },
          },
        },
      },
    };
  }
}
```

让我们密切关注 `state` 的结构。 `state`对象只是一个常规的 JavaScript 对象，它具有嵌套的属性定义。 一个对象属性包装另一个对象属性等，创建一个层级结构或“树”，`state`充当根。 遍历完整的`state`树看起来像这样，如果我们想要获取我们的`entities`：

```
state
    ->products
        ->pizzas
            ->entities
```

为了获得一个特定的属性，我们必须遍历树。 例如，我们建立自己的方式到`entities`来建立一个链条，每一层级往下连接我们从 state 到 entities。 如果我们弄错了链条中的任何链接，它就会中断也就无法创建连接。 该链中的每个链接代表对应 state 属性的引用。 因此，我们需要一个`products`的引用，然后是`pizzas`引用，最后引用到`entities`。 到了这，我们就可以访问`entities`所拥有的数据。

“引用某个属性”的含义是什么？ 为了说明这个概念，我们将创建一个 Store 类的实例，并展示我们可以访问状态对象属性的不同方式：

```ts
const store = new Store();
```

现在，store 是另一个包含`state`属性的 JavaScript 对象。 因此，其中一种方式我们可以通过熟悉的点符号来访问层级属性链。 现在让我们使用这种方法来获取我们的实体：

```ts
const entities = store.state.products.pizzas.entities;
```

这个方法确实很简单，但是当我们需要访问所需的属性时，我们会发现自己一遍又一遍地输入这个链。 对于可重用逻辑来说，这不是最有效的方法，而且对于深层次属性引用也很容易出错 - 如果某些东西未定义，它就会崩掉。

那么，如果我们能够为链中的每个环节创建快捷方式呢？ 我们可以分开创建返回`products`、`pizzas`和`entities`的函数：

```ts
const getProducts = (state) => state.products;
const getPizzas = (state) => state.pizzas;
const getEntities = (state) => state.entities;
```

注意这些功能是多么方便。 以`getEntities`为例，该函数的目的给它传参某个`state`并从该`state`中提取并返回`entities`属性。 看起来好像我们直接访问`entities`属性或直接访问该层级。 我们可以将此函数称为“状态快捷方式”，但我想将其称为状态选择器函数（state slector function）。

这里缺少的是如何直接将`state`传递给`getEntities`选择器，而不直接使用`store.state` - 否则，我们将再次依赖点表示法。 解决方案？ 我们将一个`select`方法添加到我们的`Store`类中，然后传递`state`对象到需要的选择器函数：

```ts
class Store {
  select(fn) {
    return fn(this.state);
  }
}
```

我们的`select`方法需要一个回调函数参数然后传递`state`作为参数调用它。 使用这种方法获取`entities`，我们可以按照逻辑的方式在整个选择器中传递状态，每次传递都会使我们下降到状态树的某个层级，直到遇到`entities`：

```ts
const getProducts = (state) => state.products;
const getPizzas = (state) => state.pizzas;
const getEntities = (state) => state.entities;

const entities = store.select((state) => {
  const products = getProducts(state);
  const pizzas = getPizzas(products);
  const entities = getEntities(pizzas);
  return entities;
});
```

正如我们前面所示，首先我们得到`products`。 一旦我们有`products`，我们就可以得到`pizzas`通过它再取得`entities`。 这个方法很好、很容易、当然也可以工作，但是我们可以更进一步，通过使用函数组合来创建一个可以传递给`select`的单个回调来进行声明和实现：

```ts
const getProducts = (state) => state.products;
const getPizzas = (state) => state.pizzas;
const getEntities = (state) => state.entities;

const entities$ = store.select((state) =>
  getEntities(getPizzas(getProducts(state)))
);
```

函数组合是当你通过相互嵌入函数返回单个结果时：内部函数的返回值成为最外层函数的参数，依此类推。 在这里，我们正在编写我们的选择器用来返回`entities`值。

我们已经看到，选择器函数是一个纯函数，它允许我们直接访问状态树遍历的值。 我们使用选择器来避免手动遍历状态树，反过来，我们为状态管理提供了强大的声明式函数编程。 现在选择器的概念已经很清楚了，让我们来看看为什么理解它非常重要对于掌握 NGRX 选择器。 我们继续，看看和这个相同的数据结构在 NGRx 中是什么样的。

## 特征状态选择器 Feature state selector

我们在 NGRX 中的 store 初始化为根状态 - 我们状态树的顶层。 由于我们的应用程序保持良好结构且模块化，因此我们将在状态树中创建更多条目。 我们通过使用特征模块（feature module）使我们的 Angular 应用程序保持模块化，NGRX 也为此提供支持！ 一旦我们懒加载的 Angular 模块被实例化 - 它将自身添加到我们的根 Angular 应用程序中 - 而 NGRX Store（和 Effects 也是如此！）也是如此。 这意味着一旦我们懒加载一个也有管理状态的 Angular 模块，它也会自动绑定到我们的根状态。

添加此行为非常方便简单 - 我们通过导入`StoreModule`并调用`.forFeature()`来将任何功能状态注册到功能模块中：

```ts
StoreModule.forFeature('products', reducers);
```

`.forFeature`的第一个参数包含一个表示特征状态名称的字符串，第二个参数提供了我们管理该特征状态的`reducer`。 使用`ngrx/store`提供的便捷函数`createFeatureSelector`创建特征状态的状态选择器时，特征名称起着至关重要的作用。

`createFeatureSelector`允许我们简单地通过它的特征名称来获取状态树的顶层特征状态属性：

```ts
export const getProductsState =
  createFeatureSelector<ProductsState>('products');
```

那么`createFeatureSelector`在这里发生了什么？ 首先，我们传递一个字符串，表示用于在特征模块中注册特征状态的名称。 它使用此字符串从根状态对象内查找特征状态，例如`state['products']`。

然后它返回一个类型化的选择器函数，该函数将返回对该特定状态切片的引用。

因此，`createFeatureSelector`返回一个选择器函数，该函数查找并返回指定的特征状态。 传递给它的泛型类型是我们从选择器函数获得的特征状态的类型。 在这种情况下，选择器将返回类型为`ProductState`的特征状态。 我们的`ProductState`将由各种 reducer 管理，马上我们会查看。

现在我们可以通过`getProductsState`轻松访问产品状态切片，可以在组件中使用它，如下所示：

```ts
this.store
  .select(fromStore.getProductState)
  .map((state) => state.pizzas)
  .map((pizzas) => pizza.entities);
```

为了获得我们需要的状态，我们必须依靠通过`.map()`进行映射来从顶层特征状态中提取它。 我们在每次`map`调用时都会*漫步*在`ProductState`中。 这很好，但它又是重复的，没有复用性，并且很难做单元测试。 这就是`createSelector`发挥作用的地方，我们将研究如何将它与我们新的`createFeatureSelector`结合起来。

## 状态切片选择器

由于是纯函数返回一个状态切片，选择器函数可以被组合在一起以供组件使用，它们可以由我们整体状态的各个部分组成 - 这就是状态管理变得更重要的地方，因为我们需要得到事情从一开始就是正确的。

要开始组合，我们需要定义一个起点 - 我们的最顶层特征。 通过使用`createFeatureSelector`，我们可以轻松获得对顶层状态属性的引用。 一旦我们有了这个引用，我们就可以将它与其他选择器组合起来，这些选择器指向我们的特征状态下面的状态 - 有效地遍历状态树直到我们到达期望的属性。 我们在前一节使用纯函数的一个例子中做了类似的事情。 让我们看看我们在 Store 内如何做到这一点。

我们从定义和管理状态的角度开始：reducer。 我们将使用我的[免费 NGRX 课程](https://ultimateangular.com/ngrx-store-effects)中的应用程序：

```ts
// src/products/store/reducers/index.ts
import { ActionReducerMap, createFeatureSelector } from '@ngrx/store';

import * as fromPizzas from './pizzas.reducer';
import * as fromToppings from './toppings.reducer';

export interface ProductsState {
  pizzas: fromPizzas.PizzaState;
  toppings: fromToppings.ToppingState;
}

export const reducers: ActionReducerMap<ProductsState> = {
  pizzas: fromPizzas.reducer,
  toppings: fromToppings.reducer,
};

export const getProductsState =
  createFeatureSelector<ProductsState>('products');
```

`ProductsState`表示此特征模块的特征状态。 它由另外两个状态树组成：状态树：`PizzaState`和`ToppingsState`。 我们的产品状态由我们的`reducers`（一个包含两个 reducer - pizzas 和 Toppings 的`ActionReducerMap`）管理，并且每个分别管理各个低一级状态。 让我们直观地将状态树看作是一个 JavaScript 对象：

```ts
//RootState
state = {
  //ProductState
  products: {
    //pizzaState
    pizzas: {
      entities: {},
      loaded: false,
      loading: true,
    },
    // ToppingsState
    toppings: {
      entities: {},
      loaded: false,
      loading: true,
    },
  },
};
```

为了找到我们的 pizza entities，我们需要按照我们在开始时看到的方式访问：

```
state -> products -> pizzas -> entities
```

现在我们可以引入`createSelector`来获取对状态树下面的属性的引用 - 这允许我们以简单的方式获取 pizzas。

我们已经将`getProductsState`定义为一个特征选择器，它可以给我们返回与`ProductsState`对应的状态切片。 剩下的就是把它与其他选择器合并，开始在我们的状态树上构建一个链。 这感觉就像我们有时设置了很多样板，而且我们在某些地方，但是一旦设置完成 - 我们就准备好使用它几千次而且几乎不用调整 - 选择器使用起来非常棒，适用于大数据集和多个状态。

那么，让我们深入一个层级，并使用`createSelector`跳转到另一个层级：

```ts
// src/products/store/reducers/index.ts
export interface ProductsState {
  pizzas: fromPizzas.PizzaState;
  toppings: fromToppings.ToppingsState;
}

export const getProductsState =
  careteFeatureSelector<ProductsState>('products');

export const getPizzaState = createSelector(
  getProductsState,
  (state: ProductsState) => state.pizzas
);
```

注意我们如何传递`getProductsState`作为第一个参数 - 所以我们可以从这一点开始我们的状态查找。 就这样，我们可以获取状态树更深层的属性。

`createSelector`函数最多可以接受八个选择器函数作为参数，每个函数引用不同的状态切片。 `createSelector`最后一个参数可以被当作我们的“生成器函数(projector function)”。 让我们来看看`createSelector`的 TypeScript 定义，以便在继续之前进一步掌握它：

```ts
export function createSelector<State, S1, S2, S3, Result>(
  s1: Selector<State, S1>,
  s2: Selector<State, S2>,
  s3: Selector<State, S3>,
  projector: (s1: S1, s2: S2, s3: S3) => Result
): MemoizedSelector<State, Result>;
```

我们不需要为这里的太多类型而惊慌失措 - 但让我们看看 s1，s2 和 s3。 请注意，在生成器中，我们以 s1，s2 和 s3 作为函数参数 - 按照我们提供的顺序。 这比我的第一个必须嵌套函数调用示例要好得多。 更具可读性和简洁性。

简而言之：传递给生成器函数的参数顺序和之前列出的选择器顺序一样。

生成器函数的作用非常强大。 我们可以在状态树中的任何位置请求各种状态属性，我们可以派生，转换或合并来自传递给它的状态切片的数据，并将此修改的数据作为单个对象返回 - 通常用于组件使用。 再次，它是干净简洁的 - 而且这种状态逻辑不在我们的组件内部。 我们的组件只是调用，就是这样。

在创建`getPizzaState`之前，为了在组件中获得 pizza entities，我们需要这样做：

```ts
this.store
  .select(fromStore.getProductsState)
  .map((state) => state.pizzas)
  .map((pizza) => pizza.entities);
```

然而，通过我们最新创建的`getPizzaState`函数，我们现在只需一个 map 调用：

```ts
this.store.select(fromStore.getPizzas).map((pizzas) => pizza.entities);
```

您可能猜到我们现在如何完成我们的旅程并引用这些实体 - 但我们访问的方式有点不同，通常从我们的 reducer 开始，让我们看看：

```ts
// src/products/store/reducers/pizzas.reducer.ts
export interface PizzaState {
    entities: { [id: number]: Pizzas};
    loaded: boolean;
    loading: boolean;
}

export cosnt initialState: PizzaState {
    entities: {},
    loaded: false;
    loading: false;
}

export function reducer(
    state = initialState,
    action: fromPizzas.PizzasAction
): PizzaState {
    //...switches and stuff
}

export const getPizzasEntities = (state: PizzaState) => state.entities;
export const getPizzasLoaded = (state: PizzaState) => state.loaded;
export const getPizzasLoading = (state: PizzasState) => state.loading;
```

在`PizzaState` reducer 中需要注意的是在底部输出的那些函数。 这些是状态属性选择器 - 纯函数。 这里是导出当前状级别所有其他属性的好地方，这样我们就可以在下一级轻松组合它们 - 已通过导入语句访问它们。

回到我们的顶级 reducer 文件`index.ts`，我们将编写一个选择器，可以返回我们喜爱的 pizza entities：

```ts
// src/products/store/reducers/index.ts
import * as fromPizzas from './pizzas.reducer';

export cosnt getProductsState = createFeatureSelector<ProductsState>('products');

export const getPizzaState = createSelector(
    getProductsState,
    (state: ProductsState) => state.pizzas
);

export const getPizzasEntities = createSelector(
    getPizzaState,
    fromPizzas.getPizzasEntities
);
```

我们使用`fromPizzas.getPizzasEntities`作为`createSelector`的生成器函数，它将返回对 pizza 属性 entities 的引用。

我们可以放弃遗留在组件代码中的最后一个.map（）吗？...

```ts
this.store.select(fromStore.getPizzas).map((pizza) => pizza.entities);
```

为什么不行。 我们现在可以如下获取 entities：

```ts
this.store.select(fromStore.getPizzasEntities);
```

会返回如下数据：

```ts
{
    1: { name: 'Pizza 1', id: 1},
    2: { name: 'Pizza 2', id: 2}
}
```

这很棒，而且正是我们所需要的。 然而，对于 Angular 或任何其他框架/解决方案，我们应该将这个数据结构视为一个数组。 在 Angular 的中，我们可以很好地将它用于 ngFor。

实体(entities)是一种表示通过使用唯一 ID 作为其数据引用的数据结构的方法。 它使数据查找起来非常简单，快速，可组合 - 但这是另一篇文章的故事。

那么，如果我们想要将基于实体的选择器转换为数组格式，以便通过 ngFor 使用呢？ 我们可以创建另一个选择器，并使用生成器函数将我们的数据结构映射到一个阵列，非常容易：

```ts
// src/products/store/reducers/index.ts
export const getPizzasEntities = createSelector(
  getPizzaState,
  fromPizzas.getPizzasEntities
);

export const getAllPizzas = createSelector(getPizzasEntities, (entities) => {
  return Object.keys(entities).map((id) => entities[id]);
});
```

这有几个关键的好处。 在状态管理中，我们可能希望通过 ID（标准化为实体）来查找项目，我们可以通过引用`getPizzasEntities`来实现，例如我们可以将一个路由参数 id 传递给我们的选择器，并返回单个实体。 没有循环，没有 map，只是一个对象查找。 对于某些组件，我们实际上可能需要实体，对于某些组件（如列表视图），我们对相同的数据更感兴趣，但是作为一个数组！

选择器也被记忆，这意味着它们很快，只有在需要时才会重新计算。

随着我们的任务完成，我们现在可以将这一个选择器传入我们的`store.select`中，我们完成了：

```ts
// an array of pizzas, what else could you ever ask for?
this.store.select(fromStore.getAllPizzas);
```

瞧^\_^！

## 总结

选择器刚开始掌握使用时是有些复杂，我鼓励你看看我的[例子 NGRX 应用程序](https://github.com/UltimateAngular/ngrx-store-effects-app/tree/27-testing-effects/src/products/store/selectors)，看看事情如何在一个更大的图景融合在一起。

选择器是我们如何通过引用我们数据结构的不同部分的函数来组合状态。 然后，我们可以合并它们，将它们合并，从它们中提取属性并将它们与其他属性结合起来（这对于使用实体和 id 可以从我们的状态中获取属性并将它们引入新的选择器来组成新状态而特别容易）。 可能性是无止境的，并且易于管理。 一旦我们通过选择器编写了我们的数据结构，我们就可以将其发送到我们的组件以供使用。
