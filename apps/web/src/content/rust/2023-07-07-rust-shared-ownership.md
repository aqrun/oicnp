---
title: 'Rust 共享所有权: Rc/RefCell/Arc/Mutex/RwLock'
description: 'Rust 共享所有权: Rc/RefCell/Arc/Mutex/RwLock，可以让你实现一个值拥有多个所有者、可变的全局数据和对共享数据的并发访问，同时仍可保证Rust众所周知的安全性'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'ownership', 'Arc', 'Rc', 'RefCell', 'Mutex']
---

> [Rust Shared Ownership: Rc, RefCell, Arc, Mutex, RwLock](https://levelup.gitconnected.com/exploring-multiple-ownership-in-rust-66baa5e4847b)

## 使用 Rc 共享所有权

Rc 引用计数智能指针，可以实现值的共享所有权。使用 Rc 多个指针可以引用同一个值，并且只有当最后一个指针被删除时，该值才会被释放。Rc 会记录对该值的引用数量，并在引用计数达到 0 时清理内存。如下示例：

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = a.clone();
    let c = a.clone();
}
```

使用 `clone` 来创建更多的所有者，并且它们都指向同一块内存。这并不违反 Rust 的所有权规则。

如果没有使用 Rc 克隆数据，则将创建另一个内存存储副本。Rc 的克隆操作，会得到一个新的 Rc，源代码如下:

```rust
fn clone(&self) -> Rc<T> {

    self.inner().inc_strong();

    Self::from_inner(self.ptr)
}
```

所以 Rc 的克隆并不会复制数据，只是增加了引用计数。

在所有权模型下，堆内存的生命周期与堆栈内存的生命周期绑定在一起。但 Rc 是个例外。在堆栈被销毁后，Rc 中保留的堆内存数据仍然存在。为什么?

## Box::leak()

Box::leak()是一个“泄漏”盒装值的函数，有效地赋予值一个静态生命周期。这个机制可以用来创建一个值的全局可变引用，这在某些情况下很有用。应该谨慎使用它，因为如果管理不当，它可能导致内存泄漏。

```rust
fn main() {
    let value = Box::new(42);
    let leaked_value = Box::leak(value);
    *leaked_value += 1;
    println!("Leaked value: {}", leaked_value);
}
```

代码中 `leaked_value` 是数值 42 的一个可变引用。通过使用 `Box::leak()` 方法，给值赋于静态生命周期，意味着值永远不会被释放。

Rc 也使用 `Box::leak()`，它创建的对象是从堆内存中“泄漏”出来的，不受堆栈控制。这相当于一个逃离仓库（译注：是类似于逃离木屋的游戏吗？），类似于 C/ c++中由`malloc`分配的每一块堆内存。这也会导致 Rust 编译器跳过它，允许一个内存块拥有多个所有者，直到引用计数达到 0。

## RefCell 和内部可变性（Interior Mutability）

Rc 是只读引用计数器，你不能直接获得它内部数据的可变引用，这意味着你需要使用 RefCell。它是一个允许内部可变性的智能指针。即使一个值是通过不可变引用访问的，你仍然可以使用 RefCell 获得对它的可变引用。

RefCell 强制 Rust 在运行时执行借用规则检查，如果你违反了规则，程序在运行时会崩溃。

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        children: vec![],
    }));
    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        children: vec![],
    }));

    node1.borrow_mut().children.push(node2);

    println!("Node1 children: {:?}", &node1.borrow().children);
}
```

代码中，`Node`结构体表示一个树的节点。childrend 属性是 `Rc<RefCell<Node>>` 类型的指针数组，允许共享所有权和内部可变性。然后，即使 node1 是不可变引用，我们也可以改变 node1 的子节点。

如果做如下操作会如何？

```rust
let mut node1_mut = node1.borrow_mut();
node1_mut.children.push(node2);

println!("Node1 children: {:?}", &node1.borrow().children);
```

上面代码做了同样的事，编译器检测可以通过，但执行代码就会出错。原因是借用规则不允许不可变和可变引用同时存在。

这时就需要使用花括号 `{}`:

```rust
{
    let mut node1_mut = node1.borrow_mut();
    node1_mut.children.push(node2);
}

println!("Node1 children: {:?}", &node1.borrow().children);
```

现在没有问题了。您可以看到，这里的借用规则仍然有效，只是检查时机是在运行时。

## 使用 Arc 实现线程安全的共享所有权

Arc 是“原子引用计数(Atomic Reference Counting)”的缩写，是 Rc 的线程安全版本。它允许在多个线程之间共享同一个值的所有权，并确保引用计数自动更新。这可以防止在多线程场景中使用 Rc 时可能发生的数据竞争。Arc 在性能方面不如 Rc，所以它只应该在需要线程安全时使用。示例:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(5);
    let value_clone = value.clone();
    let handle = thread::spawn(move || {
        println!("Value in thread: {}", value_clone);
    });
    handle.join().unwrap();
    println!("Value in main: {}", value);
}
```

考虑到性能要谨慎使用 Arc，只适用于跨线程访问。

## 互斥锁和读写锁线程同步

互斥锁（Mutex）和读写锁（RwLock）是 Rust 提供的同步原语，用于控制对共享可变数据的访问。互斥锁确保一次只有一个线程可以访问数据，而读写锁允许多个读取器或单个写入器访问数据。

### 互斥锁示例：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            let mut counter_guard = counter_clone.lock().unwrap();
            *counter_guard += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.lock().unwrap());
}
```

代码中，使用 Arc 在多个线程之间共享受互斥锁保护的变量`counter`。每个线程锁定互斥锁，增加计数，然后释放锁。互斥锁确保一次只有一个线程可以访问 counter，从而防止数据竞争。

### 读写锁示例

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];


    for _ in 0..3 {
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            let data_read_guard = data_clone.read().unwrap();
            println!("Data: {:?}", *data_read_guard);
        });
        handles.push(handle);
    }


    let data_clone = data.clone();
    let handle = thread::spawn(move || {
        let mut data_write_guard = data_clone.write().unwrap();
        data_write_guard.push(4);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Data: {:?}", data.read().unwrap());
}
```

代码中，我们有多个读线程和一个写线程。读线程使用`read()`获取对数据的读访问，而写线程使用`write()`获取对数据的写访问。读写锁允许多个读取器或单个写入器访问共享数据，与互斥锁相比提供了更大的灵活性。

## 总结

Rust 将动态检查最小化，这符合最小特权原则。在大多数情况下，您可以利用编译器的静态检查来确保代码符合所有权规则，从而实现安全性。在特定的情况下，Rust 为您提供了一个“逃离仓库”，允许您使用“全局”堆内存，并自动检查引用计数以确保堆内存的释放。

这些工具可以让你实现一个值拥有多个所有者、可变的全局数据和对共享数据的并发访问，同时仍可保证 Rust 众所周知的安全性。
