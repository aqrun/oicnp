---
title: 'React 受控组件和非受控组件'
description: 'React 受控组件和非受控组件有没有选择标准？'

taxonomies:
  categories: ['frontend', 'article']
  tags: ['react']
---

> 原文：[Controlled and uncontrolled form inputs in React don't have to be complicated](https://goshakkk.name/controlled-vs-uncontrolled-inputs-react/)

可能经常看到文章会讲一个很矛盾的观点一边说着“不要使用`setState`”, 而另一边内容里又大斯宣讲“`ref`s 的不好”。这就有点难以理解了，那到底有没有选择标准。

你是如何处理表单的呢？

毕竟，表单在很多 WEB 应用里是主要功能。当然表单处理在 React 中或多或少也属于核心功能。

不用害怕。接下来我们看看他们之间的区别，以及应该在什么场合使用它们。

## 非受控组件

非受控表单组件如同传统的 HTML 表单输入框：

```tsx
class Form extends Component {
  render() {
    return (
      <div>
        <input type='text' />
      </div>
    );
  }
}
```

DOM 节点保存你输入的信息，然后你可以通过 `ref` 获取它们的值。如下：

```tsx
class Form extends Component {
  handleSubmitClick = () => {
    const name = this._name.value;
    // ...
  };
  render() {
    return (
      <div>
        <input type='text' ref={(input) => (this._name = input)} />
        <button onClick={this.handleSubmitClick}>Sign up</button>
      </div>
    );
  }
}
```

也就是说，你必须在需要的时候从控件中“提取”值。提交表单时就需要这样操作。

这是实现表单输入的最简单的方法，一般会在学习 React 时这么写。

这个功能确实不够强大，接下来再看看受控组件。

## 受控组件

受控组件使用属性接收它的当前值，以及一个回调函数修改该值。可以说这是一种更符合“React 的方式”（并不不是说应该总是使用这种方式）。

```tsx
<input value={someValue} onChange={handleChange} />
```

看起来很不错，但控件的值必须保存在 `state` 中。一般渲染表单的组件会把值保存在它的 `state`:

```js
class Form extends Component {
  constructor() {
    super();
    this.state = {
      name: '',
    };
  }
  handleNameChange = (event) => {
    this.setState({ name: event.target.value });
  };
  render() {
    return (
      <div>
        <input
          type='text'
          value='this.state.name'
          onChange={this.handleNameChange}
        />
      </div>
    );
  }
}
```

> 当然，也可以保存在另一个组件的`state`中，或者分离出去，如使用 redux

每次输入一个新的字符都会调用 `handleNameChange` 方法，它获取控件最新值并更新到 `state`

<br/>
<div style="background:white;padding:20px;">
<img src="https://cdn.oicnp.com/images/js/controlled-flow.png"/>
</div>
<br/>

- 从一个空字符串开始 —— `''`
- 当输入一个 `a` 然后 `handleNameChange` 获取到 `a` 并调用 `setState`。控件就会重新渲染显示出值 `a`
- 输入 `b` 时 `handleNameChange` 获取到值 `ab` 并更新到 `state`。控件又一次重新渲染，显示出 `value="ab"`

这种流程将值的更改“推送”到表单组件，因此表单组件始终具有控件的当前值，而不需要显式地请求它。

这意味着你的数据(state)和 UI(inputs)总是同步的。state 将值提供给 input，而 input 要求表单更改当前值。

这也意味着表单组件可以立即响应控件修改，如：

- 就地反馈，如表单验证
- 除非所有字段都有有效数据，否则禁用按钮
- 强制执行特定的输入格式，比如信用卡号

但如果不需要这些功能，并且认为非控组件更方便，那就按你的喜好好了。

## 如何让一个表单元素受控呢？

当然还有其它的表单元素如：checkbox、radio、select、textarea

如果你通过 `prop` 属性设置表单元素的值，它就变成了受控组件。就这么简单

每个表单元素值属性及事件略有不同如下：

| 元素                           | 值属性               | change 事件 | 获取新值             |
| ------------------------------ | -------------------- | ----------- | -------------------- |
| &lt;input type="text"/&gt;     | value="string"       | onChange    | event.target.value   |
| &lt;input type="checkbox"/&gt; | checked={boolean}    | onChange    | event.target.checked |
| &lt;input type="radio"/&gt;    | checked={boolean}    | onChange    | event.target.checked |
| &lt;textarea /&gt;             | value="string"       | onChange    | event.target.value   |
| &lt;select /&gt;               | value="option value" | onChange    | event.target.value   |

## 总结

受控和非受控组件各有优点。具体用哪一种要根据你的使用场景进行选择。

如果你的表单非常简单，如 UI 反馈，那么使用 `ref` 引用的非受控组件完全可以。没必要去听别人说什么“不好”。

| 功能                                                                                   | 非受控组件 | 受控组件 |
| -------------------------------------------------------------------------------------- | ---------- | -------- |
| 一次性的值获取（如表单提交时）                                                         | Yes        | Yes      |
| 表单提交需要验证                                                                       | Yes        | Yes      |
| [即时验证](https://goshakkk.name/instant-form-fields-validation-react/)                | No         | Yes      |
| [有条件地禁用提交按钮](https://goshakkk.name/form-recipe-disable-submit-button-react/) | No         | Yes      |
| 强制输入格式                                                                           | No         | Yes      |
| 多个控件值组成一个数据                                                                 | No         | Yes      |
| 动态输入框                                                                             | No         | Yes      |

当然，这并不是一锤子买卖，你可以随时切换成受控组件。可以查看这个文章：[非受控组件改为受控组件并不难](https://goshakkk.name/turn-uncontrolled-into-controlled/)

最后更多关于 React 中表单的文章[点击这里](https://goshakkk.name/on-forms-react/)
