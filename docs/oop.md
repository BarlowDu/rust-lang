# Rust的面向对象特性

面向对象编程(Object-Oriented Programming,OOP)是一种模式化编程方式.对象(Object)来源于20世纪60年代的Simula编程语言.这些对象影响了Alan Kay的编程架构中对象之间的消息传递.他在1967年创造了面向对象编程这个术语来描述这种架构.关于OOP是什么有很多矛盾的定义;在一些定义下,Rust是面向对象的;其他定义下,Rust不是.在本章节中,我们会探索一些被普遍认为是面向对象的特性和这些特性是如何体现在Rust语言习惯中的.接着会展示如何在Rust中实现面向对象设计模式,并讨论这么做与利用Rust自身的一些优势的方案相比有什么取舍.

## 面向对象语言的特征

关于一个语言被称为面向对象所需的功能,在编程社区内并未达成一致意见.Rust被很多不同的编程范式影响,包括面向对象编程.面向对象编程语言所共享的一些特性往往是对象,封装,继承.让我们看一下这每一个概念的含义以及Rust是否支持他们.

### 对象包含数据和行为

由Erich Gamma,Richard Helm,Ralph Johnson和John Vlissides(Addison-Wesley Professional,1994)编写的书*Design Patterns:Elements of Reusable Object-Oriented Software*被俗称为`The Gang of Four`, 它是面向对象编程模式的目录.它这样定义面向对象编程:

> Object-Oriented programs are made up of objects.An object packages both data and the procedures that operate on that data.The Procedures are typically called methods or operations.

> 面向对象的程序是由对象组成的.一个对象包含数据和操作这些数据的过程.这些过程通常被称为方法或操作.

在这个定义下,Rust是面向对象的:结构体和枚举包含数据而`impl`块提供了结构体和枚举之上的方法.虽然带有方法的结构体和枚举并不被称为对象,但是他们提供了与对象相同的功能,参考Gang of Four中对象的定义.

### 封装隐藏了实现细节

另一个通常与面向对象编程相关的方面是封装(*encapsulation*)的思想:对象的实现细节不能被使用对象的代码获取到.所以唯一与对象交互的方式是通过对象提供的公有API;使用对象的代码无法深入到对象内部直接改变数据或行为.封装使得改变和重构对象的内部时无需改变使用对象的代码.

正如"第七章"讨论的那样:可以使用`pub`关键字来决定模块,类型,函数和方法是公有的,而默认情况下其它一切都是私有的.比如,我们可以定义一个包含一个`i32`类型的vector的结构体'AveragedCollection'.结构体也可以有一个字段,该字段保存了vector中所有值的平均值.这样,希望知道结构体中vector的平均值的人可以随时获取它,而无需自己计算.换句话说,`AveragedCollection`会为我们缓存平均值的结果,*示例:1*有`AveragedCollection`结构体的定义:

```rust
pub struct AveragedCollection{
    list:Vec<i32>,
    average:f64,
}
```
*示例:1* `AveragedCollection`结构体维护了一个整型列表和集合中所有元素的平均值.

注意,结构体自身被标记为`pub`,这样其他代码就可以使用这个结构体,但是在结构体内部的字段依然是私有的,这是非常重要的,因为我们希望保证变量被增加到列表或者被从列表删除时,也会同时更新平均值.可以通过在结构体上实现`add`,`remove`和`average`方法来做到这一点.如*示例:2*所示:

```rust
pub struct AveragedCollection{
    list:Vec<i32>,
    average:f64
}
impl AveragedCollection{
    pub fn add(&mut self,value:i32){
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self)->Option<i32>{
        let result=self.list.pop();
        match result{
            Some(value)=>{
                self.update_average();
                Some(value)
            },
            None=>None
        }
    }

    pub fn average(&self)->f64{
        self.average
    }

    fn update_average(&mut self){
        let total:i32=self.list.iter().sum();
        self.average=total as f64/self.list.len() as f64;
    }
}
```
*示例:2*

公有方法`add`,`remove`和`average`是修改`AveragedCollection`实例的唯一方式.

`list`和`average`是私有的,所以没有其他方式来使得外部的代码直接向`list`增加或者删除元素,否则`list`改变时可能会导致`average`字段不同步,`average`方法返回`average`字段的值,这使得外部的代码只能读取`average`而不能修改它.

因为我们已经封装好了`AveragedCollection`的实现细节,将来可以轻松改变类型数据结构这些方面的内容.例如,可以使用`HashSet`代替`list`字段的类型.只要`add`,`remove`和`average`公有函数的签名保持不变,使用`AveragedCollection`的代码就无需改变.相反如果使得`list`为公有,就未必都会如此了:`HashSet`和`Vec`使用不同的方法增加或移除项,所以如果要想直接修改`list`的话,外部的代码可能不得不做出修改.

### 继承,作为类型系统与代码共享

继承`Inheritance`是一个很多编程语言都提供的机制,一个对象可以定义为继承另一个对象的定义,这使其可以获得父对象的数据和行为,而无需重新定义.

如果一个语言必须有继承才能被称为面向对象语言的话,那么Rust就不是面向对象的.无法定义一个结构体继承父结构体的成员和方法.然而,如果你过去常常在你的编程工具箱中使用继承,根据你最初考虑继承的原因,Rust也提供了其他的解决方案.

选择继承有两个主要的原因.第一个是为了重用代码:一旦为一个类型实现了特定行为,继承可以对一个不同的类型重用这个实现.相反Rust代码可以使用默认trait方法来进行共享.

第二个使用继承的原因与类型系统有关:表现为子类型可以用于父类型被使用的地方.这也被称为多态(*polymorphism*),这意味着如果多种对象共享特定的属性,则可以互相替代使用.

近来继承作为一种语言设计的解决方案在很多语言中失宠了,因为其时常带有共享多于所需的代码的风险.子类不应总是共享其父类的多有特征,但是继承却始终如此,如此会使程序设计更为不灵活,并引入无意义的子类方法调用,或者由于方法实际并不适用于子类而造成错误的可能性,某些语言还只允许子类继承一个父类,进一步限制了程序设计的灵活性.

因为这些原因,Rust选择了一个不同的途径,使用trait对象替代继承.


## 为使用不同类型的值而设计的trait对象

之前我们谈到了vector只能存储同种类型元素的局限.

然而有时,我们希望用户在特定情况下能够扩展有效的类型的集合.为了展示如何实现这一点,这里将创建一个图片用户接口(Graphical User Interface,GUI)工具的例子,其它通过遍历列表并调用每一个项目的`draw`方法来将其绘制到屏幕上---此乃一个GUI工具的常见技术.我们将要创建一个叫做`gui`的库crate,它含一个GUI库的结构.这个GUI库包含一些可供开发者使用的类型,比如`Button`或`TextField`.在此之上,`gui`的用户希望创建自定义的可以绘制于屏幕上的的类型:比如,一个程序员可能会增加`Image`,另一个可能会增加`SelectBox`.

这个例子中并不会实现一个功能完善的GUI库,不过会展示其中各个部分是如何结合在一起的.编写库的时候,我们不可能知晓并定义所有其他程序员希望创建的类型.我们所知晓的是`gui`需要记录一系列不同类型的值,并需要能够对其中每一个值调用`draw`方法.这里无需知道调用`draw`方法时具体会发生什么,只需提供可供这个值调用的方法的即可.

在拥有继承的语言中,可以定义一个名为`Component`的类,该类上有一个`draw`方法.其他的类比如`Button`,`Image`和`SelectBox`会从`Component`派生并因此继承`draw`方法.它们各自都可以覆盖`draw`方法来定义自己的行为,但是框架会反所有这些类型当作是`Component`的实例,并在其上调用`draw`.不过Rust并没有继承,我们得另寻出路.

### 定义通用行为的trait

为了实现`gui`所期望拥有的行为,定义一个`Draw`trait,其包含名为`draw`的方法.接着可以定义一个存放trait对象(*trait object*)的vector.trait对象指向一个实现了我们指定trait的类型实例.我们通过指定某些指针,比如`&`引用或`Box<T>`智能指针,接着指定相关的trait(****).我们可以使用trait对象代替泛型或具体类型.任何使用trait对象的位置,Rust的类型系统会在编译时确保在此上下文中使用的值会实现某其trait对象的trait.如此但无需在编译时就知晓所有可能的类型.

之前提到过,Rust刻意不将结构体与枚举称为"对象",以便与其他语言中的对象想区别.在结构体或枚举中,结构体字段中的数据和`impl`块中的行为是分开的,不同于其他语言中将数据和行为组合进一个称为对象的概念中.trait对象将数据和行为两者相结合,从这种意义上说则其更类似其他语言中的对象.不过trait对象不同于传统的对象,因为不能向trait对象增加数据.trait对象并不像其他语言中的对象那么通过:其(trait对象)具体的作用是允许对通用行为的抽象.


*示例:#*展示了如何定义一个带有`draw`方法的trait`Draw`:

```rust
pub trait Draw{
    fn draw(&self);
}
```
*示例:3* `Draw`trait的定义

*示例:4*定义了一个存放了名叫`components`的vector的结构体`Screen`.这个vector的类型是`Box<Draw>`,此为一个trait对象:它是`Box`中任何实现了`Draw`trait的类型的替身.

```rust
#pub trait Draw{
#    fn draw(&self);
#}

pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,
}

```
*示例:4*

在`Screen`结构体上,我们将定义一个`run`方法,该方法会对其`components`上的每一个组件调用`draw`方法.如*示例:5*:
```rust
#pub trait Draw{
#    fn draw(&self);
#}

#pub struct Screen{
#    pub components:Vec<Box<dyn Draw>>,
#}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
```
*示例:5*

这与定义使用带有trait bound的泛型类型参数的结构体不同.泛型类型参数一次只能替代一个具体类型,而trait对象则允许在运行时替代多种具体类型.例如,可以定义`Screen`结构体来使用泛型和trait bound,如*示例:6*所有
```rust
pub trait Draw{
    fn draw(&self);
}

pub struct Screen<T:Draw>{
    pub components:Vec<T>,
}

impl<T> Screen<T>
    where T:Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }

}
```
*示例:6* 一种`Screen`结构体的替代方案,其run方法使用泛型和trait bound

这限制了`Screen`实例必须拥有一个全是`Button`类型或者全是`TextField`类型的组件列表.如果只需要同质(相同类型)集合,则倾向于使用泛型和trait bound,因为其定义会在编译时采用具体类型进行单态化.

>> 泛型+trait bound 

另一个方面,通过使用trait对象的方法,一个`Screen`实例可以存放一个既能包含`Box<Button>`,也能包含`Box<TextField>`的`Vec`.让我们看看它是如何工作的,接着会讲到其运行时性能影响.

### 实现trait

现在来增加一些实现了`Draw`trait的类型.我们将提供`Button`类型.

```rust
pub trait Draw{
    fn draw(&self);
}

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}

impl Draw for Button{
    fn draw(&self){
        //实际绘制按钮的代码
    }
}
```
*示例:7*

在`Button`上的`width`,`height`和`label`字段会和其他组件不同,比如`TextField`可能会有`width`,`height`,`label`和`placeholder`字段.每一个我们希望能在屏幕上绘制的类型都会使用不同的代码来实现`Draw`trait的`draw`方法来定义如何绘制特定的类型,像这里的`Button`类型.除了实现的`Draw`trait之外,比如`Button`还可能有另一种包含按钮点击如何响应的方法的`impl`块.这类方法并不适用于像`TextField`这样的类型.

