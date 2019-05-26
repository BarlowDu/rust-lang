# 智能指针

指针(*pointer*)是一个包含内存地址的变量的通用概念.这个地址引用,或"指向"(points at)一些其他数据.Rust中最常见的指针是引用(*reference*).引用以`&`符号为标志并借用了他们指向的值.除了引用数据没有任何其他特殊功能.它们也没有任何额外开销,所以应用的最多.

另一方面.智能指针(*smart pointer*)是一类数据结构,他们的表现类似指针,但是也拥有额外的元数据和功能.智能指针的概念并不为Rust所独有;其起源于C++并存在于其他语言中.Rust标准库中不同的智能指针提供了多于引用的额外功能.本章将会探索的一个例子便是**引用记数**(*reference counting*)智能指针类型,其允许数据有多个所有者.引用计数智能指针记录总共有多少个所有者,并当没有任何所有者时负责清理数据.

在Rust中,普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针;相反大部分情况下,智能指针 拥有他们指向的数据.

实际上已经出现过一些智能指针,比如`String`和`Vec<T>`,虽然当时我们并不这么称呼它们.这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们.它们也带有无数据(比如他们的容量)我额外的功能或保证(`String`的数据总是有效的UTF-8编码).

智能指针通常使用结构体实现.智能指针区别于常规结构体的显著特性在于其实现了`Deref`和`Drop`trait.`Deref`trait允许智能指针结构体实例表现的像引用一样,这样就可以编写既用于引用又用于智能指针的代码.`Drop`trait允许我们自定义当智能指针离开作用域时运行的代码.

考虑到智能指针是一个在Rust经常被使用的通用设计模式,很多库都有自己的智能指针而你也可能编写属于你自己的智能指针.

常用的智能指针:
* `Box<T>`,用于在堆上分配值
* `Rc<T>`,一个引用计数类型,其数据可以有多个所有者
* `Ref<T>`和`RefMut<T>`,通过`RefCell<T>`访问,一个运行时也不是在编译时执行的借用规则类型

另外会涉及**内部可变性**(*interior mutablility*)模式,这时不可变类型暴露出改变其内部值的API.**引用循环**(*reference cycles*)会如何泄露内存,以及如何避免

### `Box<T>`在堆上存储数据,并且可确定大小

最简单直接的智能指针是box,其类型是`Box<T>`.box允许你将一个值放在堆上而不是栈上.留在栈上的则是指向堆数据的指针.

除了数据被存储在堆上而不是栈上之外,box没有性能损失.不过也没有很多额外的功能.它们多用于如下场景:
* 当有一个在编译时未知大小的类型,而又想要在需要确切大小的上下文中使用这个类型值的时候
* 当有大量数据并希望确保数据不被拷贝的情况下转移所有权的时候
* 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型的时候

我们会在"box允许创建递归类型"部分展示第一种情况.在第二种情况中,转移大量数据的所有权可能会花费很长的时间,因为数据在栈上进行了拷贝.为了改善这种情况下的性能,可以通过box将这些数据存储在堆上.接着,只有少量的指针数据在栈上被拷贝.第三种情况被称为**trait**对象(*trait object*),见"为使用不同类型的值而设计的trait对象".

### 使用`Box<T>`在堆上储存数据

在开始`Box<T>`的用例之前,让我们熟悉一下语法和如何与储存在`Box<T>`中的值交互.
```rust
fn main(){
    let b=Box::new(5);
    println!("b={}",b);
}
```
在这个例子中,我们可以像数据是存储在栈上的那样访问box中的数据.正如任何拥有数据所有权的值那样,当像b这样在main的末尾离开作用域时,它将被释放.这个释放过程.这个释放过程作用于box本身(位于栈上)和它所指向的数据(位于堆上).

将一个单独的值存放在堆上并不是很有意义,所以像上述例子这样单独使用box并不常见.将像单个i32这样的值储存在栈上,也就是其默认存放的地方在大部分使用场景中更为合适.

### box允许创建递归类型

Rust需要在编译时知道类型占用多少空间.一种无法在编译时知道大小的类型是递归类型(*recurisve type*),其值的一部分可能是相同类型的另一个值.这种值的嵌套理论上可以无限的进行下去,所以Rust不知道递归类型需要多少空间.不过box有一个已知的大小,所以通过在循环类型定义中插入box,就可以创建递归类型了.

让我们探索一下*cons list*,一个函数式编程语言中的常见类型,来展示这个(递归类型)概念.除了递归之外,我们将要定义的cons list类型是很直白的,所以这个例子的概念在任何遇到更为复杂的涉及到递归类型的场景时都很实用.

#### cons list的更多内容
cons list是一个来源于Lisp编程语言及其方言的数据结构.在Lisp中,`cons`函数("construct function"的缩写)利用两个参数来构造一个新的列表,他们通常是一个单独的值和另一个列表.

cons函数的概念涉及到更通用的函数式编程术语;"将x与y连接"通常意味着构建一个新的容器而将x的元素放在新的容器的开关,其后则是容器y的元素.

cons list的每一项都包含两个元素:当前项的值和下一项.其最后一项值包含一个叫做`Nil`的值并没有下一项.cons list通过递归调用`cons`函数产生.代表递归的终止条件(base case)的规范名称是`Nil`,这宣布列表的终止.注意这不同与第6章的"null"或"nil"的概念,他们代表无效或缺失的值.

注意虽然函数式编程语言经常使用cons list,但是它并不是一个Rust中常见的类型.大部分在Rust中需要列表的时候,`Vec<T>`是一个更好的选择.其他更为复杂的递归数据类型确实在Rust的很多场景中很有用,不过通过以cons list作为开始,我们可以探索如何使用box毫不费力的定义一个递归数据类型.

下面的示例包含一个cons list的枚举定义.注意这还不能编译因为这个类型没有已知的大小.
```
enum List{
    Cons(i32,List),
    Nil    
}
```
>注意:出于示例的需要我们选择实现一个只存放`i32`值的cons list.也可以用泛型,来定义一个可以存放任何类型值的cons list类型.

`List`类型"有无限的大小".其原因是它的一个成员被定义为递归的:它直接存放了另一个相同类型的值.这意味着Rust无法计算为了存放`List`值到底需要多少空间.首先了解一下Rust如何决定需要多少空间来存放一个非递归类型.

#### 计算非递归类型的大小
```rust
enum Message{
    Quit,
    Move{x:i32,y:i32},
    write(String),
    ChangeColor(i32,i32,i32)
}
```
当Rust需要知道要为`Message`值分配多少空间时,它可以检查每一个成员并发现`Message::Quit`并不需要任何空间,`Message::Move` 需要足够存储两个`i32`值的空间,依此类推.因此,`Message`值所需空间等于储存其最大成员的空间大小.

与此相对当Rust编译器检查像上述示例的`List`这样的递归类型时会发生什么呢.编译器尝试计算出储存一个`List`枚举需要多少内存,并开始检查`Cons`成员,那么 `Cons`需要的空间等于`i32`的大小加上`List`的大小.为了计算List需要多少内存,它检查其成员,从`Cons`成员开始.`Cons`成员储存一个`i32`值和一个`List`值,这样的计算将无限进行下去.


#### 使用`Box<T>`给递归类型一个已知的大小
Rust无法计算出要为定义为递归的类型分配多少空间,所以编译器给出了如下的错误.这个错误也包括了有用的那这:
```
=	help:	insert	indirection	(e.g.,	a	`Box`,	`Rc`,	or	`&`)	at	some	point	to
		make	`List`	representable
```
在建议中,"indirection"意味着不同于直接储存一个值,我们将间接的储存一个指向值的指针.

因为`Box<T>`是一个指针,我们总是知道它需要多少空间:指针的大小并不会根据其指向的数据量而改变.这意味着可以将`Box`放`Cons`成员中而不是直接存放另一个`List`值.`Box`会指向另一个位于堆上的`List`值,而不是存放在`Cons`成员.从概念上讲,我们仍然有一个通过在其中"存放"其他列表创建的列表,不过现在实现这个概念的方式更像是一个项挨着另一项,而不是一项包含另一项.

我们可以修改上述`List`示例,如下:
```rust
enum List{
    Cons(i32,Box<List>),
    Nil
}

use crate::List::{Cons,Nil};

fn main(){
    let list=Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```
`Cons`成员将会需要一个`i32`的大小加上储存box指针数据的空间,`Nil`成员不储存值,所以它比`Cons`成员需要更少的空间,现在我们知道了任何`List`值需要一个`i32`加上box指针数据的大小.通过使用box,打破了这无限递归的连锁,这样编辑器就能够计算出储存`List`值需要的的大小了.

box只提供了间接存储我堆分配;他们并没有任何其他特殊功能,比如我们将会见到的其他智能指针.它们也没有这些特殊功能带来的性能损失,所以他们可以用于像cons list这样间接存储是唯一所需要功能的场景.

`Box<T>`类型是一个智能指针,因为它实现了`Deref`trait,它允许`Box<T>`值被当作引用对待.当`Box<T>`值离开作用域时,由于`Box<T>`类型`Drop`trait的实现,box所指向的堆数据也会被清除.


## 通过`Deref`trait将智能指针当作常规引用处理

实现`Deref`trait允许我们重载**解引用运算符**(*dereference operator*)`*`(与乘法运算符或glob运算符相区别).通过这种方式实现`Deref`trait的智能指针可以被当作常规引用来对待,可以编写操作引用的代码并用于智能指针.

让我们首先看看解引用运算符如何处理常规引用,接着尝试定义我们自己的类似`Box<T>`的类型并看看为何解引用运算符不能像引用一样工作.我们会探索如何实现`Deref`trait使得智能指针以类似引用的方式工作变为可能.最后,我们会讨论Rust的解引用强制多态(*deref coercions*)功能和它是如何一同处理引用或智能指针的.
>我们将要构建的`MyBox<T>`类型与真正的`Box<T>`有一个巨大的区别:我们的版本不会在堆上储存数据.这个例子重点关注`Deref`,所以其数据实际存放在何处相比其类似指针的行为来说不算重要.


### 通过解引用运算符追踪指针的值

常规引用是一个指针类型,一种理解指针的方式是将其看成指向储存在其他某处的箭头.在下而的示例中,创建了一个i32值的引用接着引用解引用运算符来跟踪所引用的数据:
```rust
fn main(){
    let x=5;
    let y=&x;
    assert_eq!(5,x);
    assert_eq!(5,*y)
}
```
变量`x`存放了一个i32值5.y等于x的一个引用.可以断言x等于5.然而,如果希望对y的值做出断言,必须使用`*y`来追踪引用所指向的值(也就是解引用).一旦解引用了y,就可以访问y所指向的整数型值并可以与5做比较.

### 像引用一样使用`Box<T>`
可以使用`Box<T>`代替引用来重写上述的代码.解引用运算符也一样能工作.如下:
```rust
fn main(){
    let x=5;
    let y=Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y)
}
```
该示例与上面示例唯一不同的地方就是将y设置为一个指向x值的box实例,而不是指向x值的引用.在最后的断言中,可以使用解引用运算符以y为引用时相同的方式追踪box指针.接下来让我们通过实现自己的box类型来探索`Box<T>`能这么做有何特殊之处.


### 自定义智能指针

为了体会默认智能指针的行为不同于引用,让我们创建一个类似于标准库提供的`Box<T>`类型的智能指针.接着会学习如何增加使用解引用运算符的功能.

从根本上说,`Box<T>`被定义为包含一个元素的元组结构体.所以下面示例以相同的方式定义了`MyBox<T>`类型.我们还定义`new`函数来对应定义于`Box<T>`的`new`函数.

```rust
struct MyBox<T>(T);
impl MyBox<T>{
    fn new(t:T)->MyBox<T>{
        MyBox(t)
    }
}
```

这里定义了一个结构体`MyBox`并声明了一个泛型参数`T`
,因为我们希望其可以存放任何类型的值.`MyBox`是一个包含T类型元素的元组结构体.`MyBox::new`函数获取一个T类型的参数被返回一个存放传入值的`MyBox`实例.

```
fn main(){
    let x=5;
    let y=MyBox::new(x);
    
    assert_eq!(5,x);
    assert_eq!(5,*y);
}
```
*尝试以使用引用和`Box<T>`相同的方式使用`MyBox<T>`*

`MyBox<T>`类型不能解引用我们并没有为其实现这个功能.为了启用`*`运算符的解引用功能,需要实现`Deref`trait.


### 通过实现`Deref`trait将某些类型像引用一样处理

`Deref`trait,由标准库提供,要求实现名为`deref`的方法,其借用`self`并返回一个内部数据的引用.下面的示例包含定义于`MyBox`之上的`Deref`实现:
```rust
use std::ops::Deref;

# struct MyBox<T>(T);

impl<T> Deref for MyBox<T>{
    type target =T;
    
    fn deref(&self)->&T{
        &self.0
    }
}
```

`type target=T;`语法定义了用此trait的关联类型.关联类型是一个稍有不同的定义泛型参数的方式,现在还无需过多的担心它;

`deref`方法体中写入了`&self.0`,这样`deref`返回了我希望通过`*`运算符访问的引用.

没有`Deref`trait的话,编译器只会解引用`&`引用类型,`deref`方法向编译器提供了获取任何实现了`Deref`trait的类型的值并调用这个类型的`deref`方法来获取一个它知道如何解引用的`&`引用的能力

>>强大的编译器

当我们在上述代码中输入`*y`时,Rust事实上在底层运行了如下代码:
```
*(y.deref())
```
Rust将`*`运算符替换为行先调用`deref`方法再进行直接引用的操作,如此我们便不用担心是不是还需要手动调用`deref`方法了.Rust的这个特性可以上我们写出行为一致的代码,无论面对的是常规引用还是实现了`Deref`的类型.

`deref`方法返回值的引用,以及`*(y.deref())`括号外边的普通解引用仍为必须的原因在于所有权.如果`deref`方法直接返回值而不是值的引用,其值(的所有权)将被移出`self`.在这里以及大部分使用解引用运算符的情况 下我们并不希望获取`MyBox<T>`内部值的所有权.

>注意,每次当我们在代码中使用`*`时,`*`运算符都被替换成了先调用`deref`方法再接着使用`*`解引用的操作,且只会发生一次,不会对`*`操作符无限递归替换,解引用出上面i32类型的值就停止.

### 函数和方法在隐式解引用强制多态
---
解引用强制多态(*deref coercions*)是Rust表现在函数或方法传参上的一种便利.其将实现`Deref`的类型的引用转换为原始类型通过`Deref`所能够转换的类型的引用.当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时,解引用强制多态将自动发生.这时会有一系列的`deref`方法被调用,把我们提供的类型转换成了参数所需要的类型.
---
解引用强制多态的加入使得Rust程序员编写函数和方法调用时无需增加过多显式使用`&`和`*`的引用我解引用.这个功能也使得我们可以编写更多同时使用于引用或智能指针的代码.

作为展示解引用强制多态的实例.
下面的示例展示了一个有着字符串slice参数的函数定义:
```rust
fn hello(name:&str){
    println!("Hello,{}",name);
}
```

可以使用字符串slice作为参数调用hello函数,比如`hello("Rust")`.解引用强制多态使得用`MyBox<String>`类型值的引用调用`hello`成功可能.如下:
```rust
#use std::ops::Deref;
#
#struct MyBox<T>(T);
#
#impl<T> MyBox<T>{
#    fn new(x:T)->MyBox<T>{
#        MyBox(x)
#    }
#}
#
#impl<T> Deref for MyBox<T>{
#    type target=T;
#
#    fn deref(&self)->&T{
#        &self.0
#    }
#}
#
#fn hello(name:&str){
#    println!("Hello,{}!",name);
#}
fn main(){
    let m=MyBox::new(String::from("Rust"));
    hello(&m);
}
```
这里使用`&m`调用`hello`函数,其为`MyBox<String>`值的引用.因为`MyBox<T>`实现了`Deref`trait,Rust可能通过`deref`调用将`&MyBox<String>`变为`&String`.标准库中提供了`String`上的`Deref`实现,其会返回字符串slice,这可以在`Deref`的API文档中看到.Rust再次调用`deref`将`&String`变为`&str`,这就符合`hello`函数的定义了.

如果Rust没有实现解引用强制多态,为了使用使用`&MyBox<String>`类型的值调用`hello`,同不得不编写如下代码.
```rust
#use std::ops::Deref;
#
#struct MyBox<T>(T);
#
#impl<T> MyBox<T>{
#    fn new(x:T)->MyBox<T>{
#        MyBox(x)
#    }
#}
#
#impl<T> Deref for MyBox<T>{
#    type target=T;
#
#    fn deref(&self)->&T{
#        &self.0
#    }
#}
#
#fn hello(name:&str){
#    println!("Hello,{}!",name);
#}
fn main(){
    let m=MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```
`(*m)`将`MyBox<String>`解引用为`String`.接着`&`和`[..]`获取了整个`String`的字符串slice来匹配`hello`的签名.没有解引用强制多态所有这些符号混在一起将更难以读写和理解.解引用强制多态使得Rust自动的帮我们处理这些转换.

当所涉及到的类型定义了`Deref`trait,Rust会分析这些类型并使用任意多次`Deref::deref`调用以获得匹配参数的类型.这些解析都发生在编译时,所以利用解引用强制多态并没有运行时惩罚.

### 解引用强制多态如何与可变性交互

类似于如何使用`Deref`trait重载不可变引用的`*`运算符,Rust提供了`Deref`trait用于可变引用的`*`运算符.

Rust当发现类型和trait实现满足三种情况的会进行解引用强制多态:
* 当`T:Deref<Target=U>`时从`&T`到`&U`.
* 当`T:DerefMut<Target=U>`时从`&mut T`到`&mut U`.
* 当`T:Deref<Target=U>`时从`&mut T`到`&mut U`.

头两种情况除了可变性之外是相同的:第一种情况表明如果有一个&T,而T实现了返回U类型的Deref,则可以直接得到&U.第二种情况表明对于可变引用也有着相同的行为.

第三种情况有些微妙:Rust也会将可变引用强制转换为不可变引用.反之是不可能的:不可变引用引用永远也不能强转为可变引用.因为根据借用规则,如果有一个可变引用,其必须是这些数据的唯一引用(否则程序将无法编译).将一个可变引用转换为不可变引用永远也不会打破借用规则.将不可变引用转换为可变引用则需要数据只能有一个不可变引用,而借用规则则无法保证这一点.因此Rust无法假设不可变转换为可变引用是可能的.

## 使用`Drop`trait运行清理代码

对于智能指针械来说第二个重要的trait是`Drop`,其允许我们在值要离开作用域时执行一些代码.可以为任何类型提供`Drop`trait的实现,同时所指定的代码被用于释放类似于文件或网络连接的资源.我们在智能指针上下文中讨论`Drop`是因为其功能几番总是用于实现智能指针.例如`Box<T>`自定义了`Drop`用来释放box所指向的堆空间.

在其他一些语言中,我们不得不记住在每次使用完智能指针实例后调用清理内存或资源的代码.如果忘记的话,运行代码的系统可能会因为负荷过重而崩溃.在Rust中,可以指定一些代码应该在值离开作用域进被执行,而编译器会自动插入这些代码.于是我们就不需要在程序中到处编写在实例结束时清理这些变量的代码---而且还不会泄露资源.

指定在值离开作用域时应该执行的代码的方式是`Drop`trait.`Drop`trait要求实现一个叫做`drop`的方法,它获取一个`self`的可变引用.为了能够看出Rust何时调用`drop`,让我们暂时使用`println!`语句实现`drop`.

下面的示例展示了唯一定制功能就是当其实例离开作用域时打印出`Dropping CustomSmartPointer!`的结构体`CustomSmartPointer`.这会演示Rust何时运行`drop`函数:
```rust
struct CustomSmartPointer{
    data:String,
}
impl Drop for CustomSmartPointer{
    fn drop(&self){
        println!("Dropping CustomSmartPointer with data`{}`!",self.data);
    }
}

fn main(){
    let c=CustomSmartPointer{data:String::from("my stuff")};
    let d=CustomSmartPointer{data:String::from("other stuff")};
    println!("CustomSmartPointers created.");
}
```

`Drop`trait包含在prelude中,无需导入它.我们在`CustomSmartPointer`上实现在`Drop`trait,并提供了一个调用`println!`的`drop`方法实现.`drop`函数体放置任何当类型实例离开作用域时期望运行的逻辑的地方.这里选择打印一些文本以展示Rust何时调用`drop`.

在`main`中,我们新建了两个`CustomSmartPointer`实例并打印出了`CustomSmartPointer created.`.在`main`的结尾,`CustomSmartPointer`的实例会离开作用域,而Rust会调用放置于`drop`方法中代码,打印出最后的信息.注意无需显示调用`drop`方法.

上述代码的执行结果:
```
CustomSmartPointers created.
Dropping CustomSmartPointer with data`other	stuff`!
Dropping CustomSmartPointer with data `my	stuff`!
```

当实例离开作用域Rust会自动调用`drop`,并调用我们指定的代码.变量以被创建时相反的顺序被丢弃,所以`d`在`c`之前被丢弃.这个例子刚好给了我们一个drop方法如何工作的可视化指导,不过通常需要指定类型所需执行的清理代码而不是打印信息.

### 通过`std::mem::drop`提早丢弃值

不幸的是,我们并不能直截了当的禁用`drop`这个功能.通常也不需要禁用`drop`;整个`Drop`trait存在的意义在于其是自动处理的.然而,有时你可能需要提早清理某个值.一个例子是当使用智能指针管理锁时;你可能希望强制运行`drop`方法来释放锁以便作用域中的其他代码可以获取锁.Rust并不允许我们主动调用`Drop`trait的`drop`方法;当我们希望在作用域结束之前就强制释放变量的话,我们应该使用的是标准库提供的`std::mem::drop`.

下面的代码示例尝试调用`Drop`trait的`drop`方法
```rust
# struct CustomSmartPointer{
#    data:String,
#}
#impl Drop for CustomSmartPointer{
#    fn drop(&mut self){
#        println!("Dropping CustomSmartPointer!");
#    }
#}

fn main(){
    let c=CustomSmartPointer{data:String::from("some data")};
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

运行这段代码会打印出如下:

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some	data`!
CustomSmartPointer dropped before the end of main.
```

`Drop`trait实现中指定的代码可以用于许多方面来使得清理变得方便和安全:比如可以用其创建我们自己的内存分配器!通过`Drop`trait和Rust所有权系统,你无需担心之后清理代码,Rust会自动考虑这些问题.

我们无需担心意外的清理掉仍在使用值,这会造成编译器错误:所有权系统确保引用总是有效的,也会确保`drop`只会在值不再使用时被调用一次.

## `Rc<T>`引用计数智能指针

大部分情况下所有权是非常明确的:可以准确的知道哪个变量拥有某个值.然而,有些情况单个值可能会有多个所有者.例如:在图数据结构中,多个边可能指向相同的结点,而这个结点从概念上讲为所有指向它的边所拥有.结点直到没有任何边指向它之前都不应该被清理.

为了启用多所有权,Rust有一个叫做`Rc<T>`的类型.其名称称为**引用计数**(*reference counting*)的缩写.引用计数意味着记录一个值引用的数理来知晓这个值是否仍在被使用.如果某个值有零个引用,就代表没有任何有效引用并可以被清理.

可以将其想象为客厅中的电视.当一个人进来看电视时,他打开电视.其他人也可以进来看电视.当最后一个人离开房间时,他关掉电视因为它不再被使用了.如果某人在其他人还在看的时候就关掉了电视,正在看电视的人肯定会抓狂的!

`Rc<T>`用于当我们希望在堆上分配一些内存供程序的多个部分读取,而且无法在编译时确定程序的哪一部分会最后结束使用它的时候.如果确实知道哪一部分是最后一个结束使用的话,就可以令其成为数据的所有者同时正常的所有权规则就可以在编译时生效.

注意`Rc<T>`只能用于间线程场景

### 使用`Rc<T>`共享数据

回到`Box<T>`定义const list的例子,这一次我们希望创建两共享第三个列表所有权的列表.其概念如下图

>https://github.com/rust-lang/book/blob/master/src/img/trpl15-03.svg

||||||
|---|---|---|---|---|
|b|3||||
|a||5|10|Nil|
|c|4||||
||||||

列表a包含5之后是10,之后是另两个列表:b从3开始而c从4开始.b和c会接上包含5和10的列表a.换句话说,这两个列表会尝试共享列表a所包含的5和10.

尝试使用`Box<T>`定义的`List`并实现不能工作,如下:

```text
enum List{
    Cons(i32,Box<List>),
    Nil,
}

use crate::List::{Cons,Nil}

fn main(){
    let a=Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b=Cons(3,Box::new(a));
    let c=Cons(4,Box::new(a));
}
```

上面展示了不能用两`Box<T>`的列表尝试共享第三个列表的所有权.编译会得出如下错误:
```text
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List`, which does not implement
   the `Copy` trait
```

`Cons`成员拥有其储存的数据,所以当创建`b`列表时,a被移到进了b这样b就拥有了a,接着当再次尝试使用a创建c的时候,这不被允许因为a的所有权已经被移动.

可以改变`Cons`的定义来存放一个引用,不过接着必须指定生命周期参数.通过指定生命周期参数,表明列表中的第一个元素都都至少与列表本身存在的一样久.例如,借用检查器不会允许的`let a=Cons(10,&Nil);`,因为临时值会在a获取其引用之前就被丢弃了.

相反,我们修改`List`的定义为使用`Rc<T>`代替`Box<T>`,如上图所示.现在第一个`Cons`变量都包含一个值和一个指向`List`的`Rc`.当创建b时,不同于获取a的所有权,这里会克隆a所包含的`Rc`,这会将引用计数从1增加到2并允许a和b共享`Rc`中数据的所有权.创建c时也会克隆a,这会将引用计数从2增加到3.每次调用`Rc:Clone`,`Rc`中数据的引用计数都会增加,直到有零个引用之前其数据都不会被清理.

```rust
enum List{
    Cons(i32,Rc<List>),
    Nil
}
use crate::List::{Cons,Nil}
use std::rc::Rc;

fn main(){
    let a=Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    let b=Cons(3,Rc::clone(&a));
    let c=Cons(4,Rc::clone(&a));
}
```

需要使用`use`语句将`Rc<T>`引入作用域因为它不在prelude中.在`main`中创建了存放5和10的列表并将其存放在a的新的`Rc<List>`中,接着当创建`b`和`c`的时,调用`Rc::clone`函数并传递`a`中的`Rc<List>`的引用作为参数.

也可以调用`a.clone`而不是`Rc::clone(&a)`,不过在这里Rust的习惯是使用`Rc::clone`.`Rc::clone`的实现并不像大部分类型的`clone`实现那样以所有数据进行深拷贝.`Rc::clone`只会增加引用计数,这并不会花费多少时间.深拷贝可能花费很长时间.通过使用`Rc::clone`进行引用计数,可以明显的区别深拷贝和增加引用计数类的克隆.当查找代码中的性能问题时,只需考虑深拷贝的克隆而无需考虑`Rc::clone`调用.

### 克隆`Rc<T>`会增加引用计数

让我们修改上面的代码以便观察创建和丢弃`a`中的`Rc<List>`的引用时引用计数的变化.

在下面的示例中,修改了`main`以便将列表`c`置于内部作用域中,这样就可以观察当`c`离开作用域时引用计数如何变化.

```rust
enum List{
    Cons(i32,Rc<List>),
    Nil
}

use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main(){

    let a=Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}",Rc::strong_count(&a));
    let b=Cons(3,Rc::clone(&a));
    println!("count after creating b = {}",Rc::strong_count(&a));
    {
        let c=Cons(4,Rc::clone(&a));
        println!("count after creating c = {}",Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}",Rc::strong_count(&a));
}
```

在程序中每个引用计数变化的点,会打印出引用计数,其值可以通过调用`Rc::strong_count`函数获得.这个函数叫做`strong_count`而不是`count`是因为`Rc<T>`也有`weak_count`;在`避免引用循环`部分会讲解`weak_count`的用途.

>`Rc<T>`允许通过不可变引用来只读的在程序的多个部分共享数据.如果`Rc<T>`也允许多个可变引用,则会违反第四章讨论的借用规则之一:相同位置的多个可变借用可能造成数据竞争和不一致.不过可以修改数据是非常有用的!在下一部分我们将讨论内部可变性模式和`RefCall<T>`类型,它可以与`Rc<T>`结合使用来处理不可变性的限制.

---
在数组中,数组的slice,元素被引用那么数组将是
---