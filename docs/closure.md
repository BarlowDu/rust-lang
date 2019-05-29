# 闭包:可以捕获环境的匿名函数

## 闭包类型推断和注解

闭包不要求像`fn`函数那样在参数和返回值上注明类型.函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分.严格的定义这些接口对于保证所有人都认同函数使用和返回值类型来说是很重要的.但是闭包并不用于这样暴露在外的接口:他们储存在变量中并被使用,不用命名他们或暴露给库的用户调用.

闭包通常很短并只与对应相对任意的场景较小的上下文中.在这些有限制的上下文件中,编译器能可靠的推断参数和返回值的类型,类似于它是如何能够推断大部分变量的类型一样.

强制在这些小的匿名函数中注明类型 是很恼人的,并且与编译器书籍的信息存在大量的重复.

类似于变量,如果相比严格的必要性你更希望增加明确性并变得啰嗦,可以选择增加类型注解.如下
```rust
use std::thread;
use std::time::Duration;

let expensive_closure=|num:i32|->i32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}
```
*示例:1*

有了类型注解闭包的语法就更类似函数了.如下是一个对其参数加一的函数的定义与拥有相同行为闭包语法的纵向对比.这里增加了一些空格来对齐相应部分.这展示了闭包语法如何类似于函数语法,除了使用竖线而不是括号以及几个可选的语法之外:
```text
fn add_one_v1   (x:u32)->u32{x+1}
let add_one_V2= |x:u32|->u32{x+1};
let add_one_v3= |x|         {x+1};
let add_one_v4=|x|          x+1;
```

第一行展示了一个函数定义,而第二行展示了一个完整标注的闭包定义,第三行闭包定义中省略了类型注解,而第4行去掉了可选的大括号,因为闭包体只有一行.这些都是有效的闭包定义,并在调用时产生相同的行为.

闭包定义会为每个参数和返回值推断一个具体类型,例如*示例:2*中展示了仅仅将参数作为返回值的简短的闭包定义.除了作为示例的目的这个闭包并不是很实用,注意其定义并没有增加任何类型注解:如果尝试调用闭包两次,第一次使用`String`类型作为参数而第二次使用`u32`,则会得到一个错误:
```text
let example_closure=|x| x;

let s=example_closure(String::from("hello"));
let n=example_closure(5);
```

编译器给出如下错误:

```

```

第一次使用`String`值调用`example_closure`时,编译器推断`x`和此闭包返回值类型为`String`.接着这些类型被锁定进闭包`example_closure`中,如果尝试对同一闭包使用不同类型则会得到类型错误.

## 使用带泛型和`Fn`trait的闭包

可以创建一个存放闭包和调用闭包结果的结构体.该结构体只会在需要结果时执行闭包,并会缓存结果值,这样余下的代码就不必再负责保存结果并可以复用该值.你可能见过这种模式被称为*memoization*或`lazy evaluation`.

为了让结构体存放闭包,我们需要指定闭包的类型,因为结构体定义需要知道其每一个字段的类型.每一个闭包实例有其自己独有的匿名类型:也就是说,即使两个闭包有着相同的签名,他们的类型仍然被认为是不同.为了定义使用闭包的结构体,枚举,或函数参数,需要使用泛型和trait bound.

`Fn`系列trait由标准库提供.所有闭包都实现了trait`Fn`,`FnMut`,`FnOnce`中的一个.在"闭包会捕获其环境"部分我们会讨论这些trait的区别de这个例子中可以使用`Fn`trait.

为了满足`Fn`trait bound我们增加了代表闭包所必须的参数和返回值类型的类型.在这个例子中闭包有一个`u32`的参数并返回一个`u32`,这样所指定的trait bound就是`Fn(u32)->u32`.

*示例:2*展示了存放闭包和一个Option结果值的Cacher结构体的定义:
```rust
struct Cacher<T>
    where T:Fn(u32)->u32{
        calculation:T,
        value:Option<u32>
    }
```
*示例:2*

结构体`Cacher`有一个泛型`T`的字段`calculation`.`T`的trait bound指定了`T`是一个使用`Fn`的闭包.任何我们希望储存到`Cacher`实例的`calculation`字段的闭包必须有一个`u32`参数(由`Fn`之后的括号的内容指定)并必须返回一个`u32`(由`->`之后的内容).

> 注意函数也都实现了这三个`Fn`trait.如果不需要捕获环境中的值,则可以使用实现了`Fn`trait的函数而不是闭包.

`value`是`Option<i32>`类型的.在执行闭包之前.`value`将是`None`.如果使用`Cacher`的代码请求闭包并将结果储存在`value`字段`Some`成员中,接着如果代码再次请求闭包的结果,这时就不再执行闭包,而是会返回存放在`Some`成员中的结果.

```rust
struct Cacher<T>
    where T:Fn(u32)->u32
{
        calculation:T,
        value:Option<u32>
}

impl<T> Cacher<T>
    where T:Fn(u32)->u32
{
    fn new(calculation:T)->Cacher<T>{
        Cacher{
            calculation,
            value:None,
        }
    }

    fn value(&mut self,arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v=self.calculation(arg);
                self.value=Some(v)
                v
            }
        }
    }
}
```
*示例:3*

`Cacher`结构体的字段是私有的,因为我们希望`Cacher`管理这些值而不是任由调用代码潜在的直接改变他们.

`Cacher::new`函数获取一个泛型类型参数`T`,它定义于`impl`块上下文中并与`Cacher`结构体有着相同的trait bound.`Cacher::new`返回一个在`calculation`字段中存放了指定闭包和在`value`字段中存放了`None`值的`Cacher`实例,因为我们还未执行闭包.

## `Cacher`实现的限制

值缓存是一种更加广泛的实用行为,我们可能希望在代码中的其他闭包中也使用他们.然而,目前`Cacher`的实现存在两个小问题,这使得我们在不同上下文中复用变得很困难.

第一个问题是`Cacher`实例假设对于`value`方法的任何`arg`参数值总是返回相同的值.第二个问题是它的应用被限制为只接受获取一个`u32`值并返回一个`u32`值闭包.比如说,我们可能需要能够缓存一个获取字符串slice并返回`usize`值的闭包结果.请尝试引用更多泛型参数来增加`Cacher`功能的灵活性。

## 闭包捕获其环境

闭包还有一个函数所没有的功能:他们可以捕获其环境并访问其被定义的作用域的变量.

*示例:4*有一个储存在`equal_to_x`变量中闭包的例子,它使用了闭包环境中的变量`x`:
```rust
fn main(){
    let x=4;
    let equal_to_x=|z|z==x;
    let y=4;
    assert!(equal_to_x(y));
}
```
*示例:4*

这里,即便`x`并不是`equal_to_x`的一个参数,`equal_to_x`闭包也被允许使用变量`x`,因为它与`equal_to_x`定义于相同的使用域.


当闭包从环境中捕获一个值,闭包会在闭包体中储存这个值以供使用.这会使用内存并生产额外的开销,在更一般的场景中,当我们不需要闭包来捕获环境时,我们不希望产生这些开销.因为函数从未允许捕获环境,定义和使用函数也就从不会有这些额外开销.

闭包可以通过三种方式捕获其环境,他们直接对应函数的三种获取方式:获取所有权,可变借用和不可变借用.这三种捕获值的方式被编码为如下三个`Fn`trait:
* `FnOnce`消费从周围作用域捕获的变量,闭包周围的作用域被称为环境,*environment*.为了消费捕获到的变量,闭包必须获取其所有权并在定义闭包时将其移动进闭包.基名称`Once`部分代表了闭包不能多次获取相同变量的所有权的事实,所以它只能被调用一次.
* `FnMut`获取可变的借用值所以可以改变其环境.
* `Fn`从其环境获取不可变的借用值.

当创建一个闭包时,Rust根据其如何使用环境中变量来推断我们希望如何引用环境.由于所有闭包都可以被调用至少一次,所以所有闭包都实现了`FnOnce`.那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了`FnMut`,而不需要对被捕获变量进行可变访问的闭包则也实现了`Fn`.在*示例:4*中 `equal_to_x`闭包不可变借用了`x`(所以`equal_to_x`具有`Fn`trait).因为闭包体只需要读取`x`的值.

如果你希望强制闭包获取其使用的环境值的所有权,可以在参数列表前使用 `move`关键字.这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用.

*示例:5*在闭包中定义中增加`move`关键字并使用vector代替整型,因为整型可以被拷贝而不是移动;注意这些代码还不能编译:
```text
fn maint(){
    let x=vec![1,2,3];
    let equal_to_x=move |z| z==x;
    println!("can't use x here:{:?}",x);
    let y=vec![1,2,3];
    assert!(equal_to_x(y));
}
```
*示例:5*

complie error
```text
error[E0382]: use of moved value: `x`
--> src/main.rs:6:40
|
4 | let equal_to_x = move |z| z == x;
| -------- value moved (into closure) here
5 |
6 | println!("can't use x here: {:?}", x);
| ^ value used here after move
|
= note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
implement the `Copy` trait
```

`x`被移动进了闭包,因为闭包使用`move`关键字定义.接着闭包获取`x`的所有权,同时`main`就不再允许在`println!`语句中使用`x`了,去掉`println!`即可修复问题.

大部分需要指定一个`Fn`系列trait bound的时候,可以从`Fn`开始,而编译器会根据闭包体中的情况告诉你是否需要`FnMut`或`FnOnce`.


