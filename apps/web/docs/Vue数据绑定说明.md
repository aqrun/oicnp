# Vue 数据绑定说明文档

## 概述

本文档详细解释了 `demo-solar-month` 模块中 Vue.js 的数据绑定机制。

## Vue 数据绑定核心概念

### 1. 响应式数据（Reactive Data）

Vue 的核心特性是**响应式数据绑定**。当数据改变时，视图会自动更新。

```javascript
data: {
  month: {
    name: '',
    weeks: []
  },
  weeks: weekHeads
}
```

- `data` 对象中的所有属性都会被 Vue 转换为响应式的
- 当这些属性的值改变时，使用这些属性的模板部分会自动更新

### 2. 模板语法

#### 2.1 插值表达式 `{{ }}`

```html
<div class="month">{{ month.name }}</div>
```

- `{{ month.name }}` 会显示 `data.month.name` 的值
- 当 `month.name` 改变时，这里会自动更新

#### 2.2 列表渲染 `v-for`

```html
<li v-for="w in weeks">{{ w.name }}</li>
```

- `v-for="w in weeks"` 遍历 `data.weeks` 数组
- `w` 是当前循环项
- 会为数组中的每个元素创建一个 `<li>` 标签

**嵌套循环：**
```html
<ul v-for="week in month.weeks">
  <li v-for="d in week.days">{{ d.day }}</li>
</ul>
```

- 外层循环遍历 `month.weeks`（每一周）
- 内层循环遍历 `week.days`（每一天）

#### 2.3 属性绑定 `:attribute` 或 `v-bind:attribute`

```html
<li :class="{'weekend': w.isWeekend}">
```

- `:class` 是 `v-bind:class` 的简写
- 动态绑定 CSS 类
- 当 `w.isWeekend` 为 `true` 时，添加 `weekend` 类

**复杂条件绑定：**
```html
:class="{
  'holiday': d.holiday, 
  'today': d.isToday,
  'gray': !d.isCurrentMonth
}"
```

- 根据多个条件动态添加不同的 CSS 类

#### 2.4 条件渲染 `v-if` / `v-else-if` / `v-else`

```html
<u v-if="d.holiday">{{ d.holiday.isWork ? '班' : '休' }}</u>
<u v-else-if="d.isToday">今</u>
```

- `v-if`：条件为真时渲染元素
- `v-else-if`：前面的条件为假时，检查这个条件
- 可以使用三元运算符：`条件 ? 值1 : 值2`

#### 2.5 事件绑定 `@event` 或 `v-on:event`

```html
<div @click="prevMonth">上月</div>
```

- `@click` 是 `v-on:click` 的简写
- 点击时调用 Vue 实例的 `prevMonth` 方法

## 数据流向

### 初始化流程

1. **创建 Vue 实例**
   ```javascript
   new Vue({
     el: '#demo-solar-month',
     data: { ... },
     methods: { ... }
   })
   ```

2. **挂载到 DOM**
   - Vue 找到 `id="demo-solar-month"` 的元素
   - 解析模板中的 Vue 指令

3. **执行 mounted 钩子**
   ```javascript
   mounted: function() {
     this.compute();  // 初始化数据
   }
   ```

4. **渲染视图**
   - Vue 根据 `data` 中的数据渲染模板
   - 建立数据与视图的绑定关系

### 数据更新流程

1. **用户操作**（如点击"上月"按钮）
   ```javascript
   @click="prevMonth"
   ```

2. **调用方法**
   ```javascript
   prevMonth: function() {
     month = month.next(-1);  // 改变月份
     this.compute();           // 重新计算数据
   }
   ```

3. **更新响应式数据**
   ```javascript
   that.month.name = month.toString();
   that.month.weeks = weeks;
   ```

4. **Vue 自动检测变化**
   - Vue 检测到 `month.name` 和 `month.weeks` 的变化

5. **自动更新视图**
   - `{{ month.name }}` 自动更新显示新的月份名称
   - `v-for="week in month.weeks"` 自动重新渲染列表

## 数据结构说明

### weeks（星期表头）

```javascript
weeks: [
  { isWeekend: false, name: "周一" },
  { isWeekend: false, name: "周二" },
  ...
  { isWeekend: true, name: "周六" },
  { isWeekend: true, name: "周日" }
]
```

### month.weeks（月份数据）

```javascript
month: {
  name: "2024年1月",
  weeks: [
    {
      days: [
        {
          day: 1,                    // 日期数字
          holiday: null,              // 节假日信息（如果有）
          isCurrentMonth: true,       // 是否当前月
          isToday: false,            // 是否今天
          isWeekend: false,          // 是否周末
          text: "十一月二十 甲子",    // 显示文本（农历等）
          moon: false,               // 是否月相
          moonIndex: 0               // 月相索引
        },
        // ... 更多日期
      ]
    },
    // ... 更多周
  ]
}
```

## 关键方法说明

### compute() 方法

这是核心的数据计算方法：

1. **获取月份信息**
   ```javascript
   that.month.name = month.toString();
   ```

2. **遍历每一周和每一天**
   ```javascript
   var monthWeeks = month.getWeeks(weekStart);
   for (var i = 0; i < monthWeeks.length; i++) {
     var weekDays = monthWeeks[i].getDays();
     for (var x = 0; x < weekDays.length; x++) {
       // 处理每一天的数据
     }
   }
   ```

3. **构建数据对象**
   - 使用 Tyme 库获取公历、农历、节假日等信息
   - 构建符合模板需要的数据结构

4. **更新响应式数据**
   ```javascript
   that.month.weeks = weeks;
   ```
   - 这一步会触发 Vue 的响应式更新
   - 模板会自动重新渲染

### prevMonth() 和 nextMonth() 方法

这两个方法处理月份切换：

```javascript
prevMonth: function() {
  month = month.next(-1);  // 改变月份对象
  this.compute();           // 重新计算并更新数据
}
```

- 使用 Tyme 库的 `next()` 方法获取上/下一个月
- 调用 `compute()` 重新计算数据
- Vue 自动检测变化并更新视图

## Vue 响应式原理（简化说明）

1. **数据劫持**：Vue 使用 `Object.defineProperty` 监听 `data` 对象的变化

2. **依赖收集**：模板中使用数据时，Vue 会记录哪些视图依赖于哪些数据

3. **变化通知**：当数据改变时，Vue 通知所有依赖这个数据的视图更新

4. **虚拟 DOM**：Vue 使用虚拟 DOM 来高效地更新真实 DOM

## 总结

Vue 的数据绑定机制让开发者只需要：

1. **定义数据**：在 `data` 中定义需要的数据
2. **编写模板**：使用 Vue 指令（`v-for`、`v-if`、`{{ }}` 等）编写模板
3. **更新数据**：在方法中更新 `data` 中的数据
4. **自动更新**：Vue 自动更新视图

这就是 Vue 的核心优势：**数据驱动视图**，开发者不需要手动操作 DOM。

