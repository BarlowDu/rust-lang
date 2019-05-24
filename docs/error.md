Rust对可靠性的执着也延伸到了错误处理.错误对于软件来说是不可避免的,所以Rust有很多特性来处理出现错误的情况.在很多情况下,Rust要求你承认出错的可能性并在编译代码之码之前就采取行动.这些要求使得程序更为健壮,它们确保了你会在将代码部署到生产环境之前就发现错误并正确地处理它们!

Rust将错误组合成两个主要类别:可恢复错误(recoverable) 和 不可恢复错误(unrecoverable).可恢复错误通常代表向用户报告错误和重试操作是合理的情况,比如未找到文件.不可恢复错误通常是bug的同义词,比如尝试访问超过数组结尾的位置.

大部分语言并不区分这两类错误,并采用类似异常这样方式统一处理它们.Rust并没有异常.相反,对于可恢复错误有Resutl<T,E>值,以及panic!,它在遇到不可恢复错误时停止程序执行.这一章会首先介绍panic!调用,接着会讲到如何返回Result<T,E>.此外,我们将探讨决定是尝试从错误中恢复还是停止执行时的注意事项.


## panic!与不可恢复的错误

突然有一天,代码出问题了,而你对此束手无策.对于这种情况,Rust有panic!宏.当执行这个宏时,程序会打印出一个错误信息,展开并清理栈数据,然后接着退出.出现这种情况的场景通常是检测到一些类型的bug而且程序员并不清楚该如何处理它.

> ### 对应panic时的栈展开或终止
>
>当出现panic时,程序默认会开始展开(unwinding),这意味着Rust会回溯栈并清理它遇到的第一个函数,不过这个回溯并清理的过程有很多工作.别一种选择是直接终止(abort),这会不清理数据就退出程序.那么程序所使用的内存需要操作系统来清理.如果你需要项目的最终二进制文件越小越好,panic时通过在Cargo.toml的[profile]部分增加panic='abort',可以由展开切换为终止.例如,如果你想要在release模式中panic时直接终止
> ```toml
> [profile.release]
> panic = 'abort'
> ```

## 使用panic!的backtrace

我们可以设置`RUST_BACKTRACE`环境变量来得到一个backtrace.backtrace是一个执行到目前位置所有被调用的函数的列表.Rust的backtrace跟其他语言中的一样:阅读backtrace的关键是从头开始读直到发现你编写的代码.这些行可能包含核心Rust代码,标准库代码或用到的crate代码.

## Result与可恢复的错误

大部分错误并没有严重到需要程序完全停止执行.有时,一个函数会因为一个容易理解并做出反应的原因失败.例如,如果尝试打开一个文件不过由于文件不存在而失败,此时我们可能想要创建这个文件而不是终止进程.

```rust
enum Result<T,E>{
    Ok(T),
    Err(E)
}
```
T和E是泛型类型参数.T代表成功时返回的Ok成员中的数据的类型,而E代表失败时返回的Err成员中的错误的类型.因为Result有这些泛型类型参数,我们可以将Result类型和标准库中为其定义的函数用于很多不同的场景,这些情况中需要返回的成功值和失败值可能会各不相同.

```rust
use std::fs::File;
fn main(){
    let f=File::open("hello.txt");
}
```

如何知道File::open返回的是一个Result呢?我们可以查看标准库文档,或者可以直接问编译器!如果给f某个我们知道不是返回值类型的类型注解,接着尝试编译代码,编译器会告诉我们类型不匹配.然后错误信息会告诉我们f的类型应该是什么.让我们试试!

File::open函数的返回值类型是Result<T,E>.这里泛型T放入了成功值的类型std::fs::File,它是一个文件句柄.E被用在失败值上时E的类型是std::io::Error.

我们需要在如下代码中增加根据File::open返回值进行不同的处理逻辑.

```rust
use std::fs::File;

fn main(){
    let f=File::open("hello.txt");

    let f=match f{
        Ok(file)=>file,
        Err(error)=>{
            panic!("There was a problem opening the file:{:?}",error)
        }
    }
}
```

注意与Option枚举一样,Result枚举和其成员也被导入到了prelude中,所以就不需要在match分支中的Ok和Err之前指定Result::.

这里我们告诉Rust当结果是Ok时,返回Ok成员中的file值,然后将这个文件句柄赋值给变量f.match之后,我们可以利用这个文件句柄来进行读写.

match的另一个分支处理从File::open得到Err值的情况.在这种情况下,我们选择调用panic!宏.如果当前目录没有一个叫做hello.txt文件,当运行这段代码时会看到如下来自panic!宏的输出.

### 匹配不同的错误

上面的代码不管File::open是因为什么原因失败都会panic!.我们真正希望的是对不同的错误原因采取不同的行为:如果File::open是因为文件不存在失败,我们希望创建这个文件并返回新文件的句柄.如果File::open因为任何其它原因失败,例如没有打开文件权限,我们依然希望像上面一样panic!.下面的代码中match增加了别一个分支:
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f=File::open("hello.txt");

    let f=match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Tried to create file but was a problem:{:?}",e)
            },
            other_error=>panic!("There was a problem opening the file:{:?}",error)
        }
    }
}
```

`File::open`返回的Err成员中的值类型io::Error,它是一个标准库中提供的结构体,这个结构体有一个返回io::ErrorKind值的kind方法可供调用.`io::ErrorKind`是一个标准库提供的枚举,它的成员对应io操作可能导致的不同错误类型.

这里有好多match!match确实很强大,不过也非常的基础.Result<T,E>有很多接受闭包的方法,并采用match表达式实现,一个更老练的Rustacean可能会这么写:
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f=File::open("hello.txt").map_error(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or else(|error|{
                panic!("Tried to create file but there was a problem:{:?}",error);
            })
        }else{
            panic!("There was a problem opening the file:{:?}",error);
        }
    })
}

```

### 失败时`panic!`的简写:`unwrap`和`except`

`mathc`能够胜任它的工作,不过它可能有点冗长并且不总是能很好的表明其意图.Result<T,E>类型定义了很多辅助方法来帮助处理各种情况.其中之一叫做`unwrap`,它的实现就类似于上面示例的`match`语句.如果`Result`成员是`Ok`,`unwrap`会返回`Ok`中的值.如果`Result`是成员`Err`,`unwrap`会为我们调用`panic!`.

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

如果调用这段代码时不存在*hello.txt*,我们将会看到一个`unwrap`调用`panic!`时提供的错误信息
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

还有另一个类似于`unwrap`的方法它还允许我们选择`panic!`的错误信息:`except`.使用`except`而不是`unwrap`并提供一个好的错误信息可以表明你的意图并更易于追踪`panic`的根源.`except`的语法看起来像这样:
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

### 传播错误
当编写一个其实现会调用一些可能会失去失败的操作的函数时,除了在这个函数中处理错误外,还可以选择让调用者知道这个错误并决定该如何处理.这被称为**传播**(*propagating*)错误,这样能更好的控制代码调用,因为比起你代码所拥有的上下文,调用者可能拥有更多的信息或逻辑来决定应该如何处理错误.


传播错误的简写:`?`

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_user_name_from_file()->Result<String,io::Error>{
    let mut f=File::open("hello.txt")?;
    let mut s=String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

```

### `?`只能被用于返回`Result`的函数

`?`只能被用于返回值类型为`Result`的函数,因为它被定义为`match`表达式有着完全相同的工作方式.`match`与`return Err(e)`部分要求返回值类型是`Result`,所以函数的返回值必须是`Result`才能与这个`return`相兼容.

让我们看看在`main`函数中使用`?`会发生什么,如果你还记得的话其返回值类型是`()`:

### `panic!`还是不`panic!`

那么,该如何决定何时应该`panic!`以及何时应该返回`Result`呢?如果代码panic,就没有恢复的可能.你可以选择对任何错误场景都调用`panic!`,不管是否有可能恢复,不过这样就是你代替调用者决定了这是不可恢复的.选择返回`Result`值的话,就将选择权交给了调用者,而不是代替他们做决定,调用者可能会选择符合他们场景的方式尝试恢复,或者也可能干脆就认为`Err`是不可恢复的,所以他们也可能会调用`panic!`并将可恢复的错误变成了不可恢复的错误,因此返回`Result`是定义可能会失败的函数一个好的默认选择.

有一些情况panic比返回`Result`更为合适,不过他们并不常见.让我们讨论一下为何在示例,代码原型和测试中,以及那些人们认为不会失败的而编译器不这么看的情况下,panic是合适的.

---
在多线程下panic是否会终止进程
---
