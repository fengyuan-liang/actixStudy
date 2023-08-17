# rust web开发

## 1. 构建原生web服务器





![image-20230806222420080](https://cdn.fengxianhub.top/resources-master/image-20230806222420080.png)





![image-20230806222809669](https://cdn.fengxianhub.top/resources-master/image-20230806222809669.png)







## 2. actix入门

![image-20230807012257467](https://cdn.fengxianhub.top/resources-master/image-20230807012257467.png)

在Actix中支持两类并发

>1. 异步IO（IO多路复用）
>2. 多线程并行：即OS线程数与逻辑CPU数量相同

接下来我们要添加三个路由

![image-20230807012528474](https://cdn.fengxianhub.top/resources-master/image-20230807012528474.png)







### 2.1 内存中添加数据

```shell
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"First course"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"Second course"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"Third course"}'
```

测试数据

```shell
$ curl  localhost:3000/courses/1
{"code":200,"data":[{"teacher_id":1,"id":1,"name":"First course","time":"2023-08-09T13:16:55.348653800"},{"teacher_id":1,"id":2,"name":"Third course","time":"2023-08-09T13:16:58.842984200"},{"teacher_id":1,"id":3,"name":"Third course","time":"2023-08-09T13:17:03.828113800"},{"teacher_id":1,"id":4,"name":"Second course","time":"2023-08-09T13:17:11.449509100"}]}
```

测试接口`get_course_detail`

```shell
$ curl localhost:3000/courses/1/1
{"code":200,"data":{"teacher_id":1,"id":1,"name":"First course","time":"2023-08-09T13:20:09.925247900"}}
```



### 2.2 启动pgSQL

```yaml
version: '3'
services:
  postgresql:
    image: registry.cn-hangzhou.aliyuncs.com/zhengqing/postgres:14.5                    # 镜像'postgres:14.5'
    container_name: postgresql                                                          # 容器名为'postgresql'
    restart: unless-stopped                                                             # 指定容器退出后的重启策略为始终重启，但是不考虑在Docker守护进程启动时就已经停止了的容器
    # 设置环境变量,相当于docker run命令中的-e
    environment:
      TZ: Asia/Shanghai
      LANG: en_US.UTF-8
      POSTGRES_DB: postgres
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: 123456
      ALLOW_IP_RANGE: 0.0.0.0/0 # 允许所有ip访问
    # 数据卷挂载路径设置,将本机目录映射到容器目录
    volumes:
      - "./postgresql/data:/var/lib/postgresql/data"
    # 映射端口
    ports:
      - "5432:5432"
```

```shell
docker-compose up -d
```

使用

```shell
# 进入容器
docker exec -it postgresql bash
# 登录
psql -U postgres -W
# 查看版本
select version();
```

### 2.3 数据库操作

数据库连接

```shell
DATABASE_URL=postgres://postgres:123456@localhost:5432/yx
//                        账号      密码      ip     端口   数据库名称
```

常用命令

```sql
连接数据库, 默认的用户和数据库是postgres
psql -U user -d dbname

切换数据库,相当于mysql的use dbname
\c dbname
列举数据库，相当于mysql的show databases
\l
列举表，相当于mysql的show tables
\dt
查看表结构，相当于desc tblname,show columns from tbname
\d tblname

\di 查看索引 

创建数据库： 
create database [数据库名]; 
删除数据库： 
drop database [数据库名];  
*重命名一个表： 
alter table [表名A] rename to [表名B]; 
*删除一个表： 
drop table [表名]; 

*在已有的表里添加字段： 
alter table [表名] add column [字段名] [类型]; 
*删除表中的字段： 
alter table [表名] drop column [字段名]; 
*重命名一个字段：  
alter table [表名] rename column [字段名A] to [字段名B]; 
*给一个字段设置缺省值：  
alter table [表名] alter column [字段名] set default [新的默认值];
*去除缺省值：  
alter table [表名] alter column [字段名] drop default; 
在表中插入数据： 
insert into 表名 ([字段名m],[字段名n],......) values ([列m的值],[列n的值],......); 
修改表中的某行某列的数据： 
update [表名] set [目标字段名]=[目标值] where [该行特征]; 
删除表中某行数据： 
delete from [表名] where [该行特征]; 
delete from [表名];--删空整个表 
创建表： 
create table ([字段名1] [类型1] ;,[字段名2] [类型2],......<,primary key (字段名m,字段名n,...)>;); 
\copyright     显示 PostgreSQL 的使用和发行条款
\encoding [字元编码名称]
                 显示或设定用户端字元编码
\h [名称]      SQL 命令语法上的说明，用 * 显示全部命令
\prompt [文本] 名称
                 提示用户设定内部变数
\password [USERNAME]
                 securely change the password for a user
\q             退出 psql


```



### 2.4 数据库CRUD

添加课程

```shell
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "id":7, "name":"First course"}'
```





打包脚本

```shell
$ docker run -it --rm \
 -v $PWD:/workdir \
 -v ~/.cargo/git:/root/.cargo/git \
 -v ~/.cargo/registry:/root/.cargo/registry \
 registry.gitlab.com/rust_musl_docker/image:stable-latest \
 cargo build --release -vv --target=x86_64-unknown-linux-musl
```

压测了其中一个接口，qps能到5000以上

![image-20230810235558695](https://cdn.fengxianhub.top/resources-master/image-20230810235558695.png)

也没有太多接口超时，rust的异步db请求还是挺强的

![image-20230810235604293](https://cdn.fengxianhub.top/resources-master/image-20230810235604293.png)



### 2.5 统一异常处理

在java中我们经常进行统一的异常处理，在controller处理异常的逻辑

在rust中也可以进行统一错误处理

![image-20230811224946379](https://cdn.fengxianhub.top/resources-master/image-20230811224946379.png)

编程语言常用的两种错误处理方式：

- 异常（java）
- 返回值（Rust、golang）

rust希望开发者显式的处理错误，因此，可能出错的函数返回Result枚举类型

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

在rust中可以使用`?`简化抛出异常

这里`?`的作用我们总结一下：

- 如果`Result`是`Ok`：`Ok`中的值就是表达式的结果，然后继续执行程序
- 如果`Result`是`Err`：`Err`就是`整个函数`的返回值，就像使用了`return`一样

```rust
use std::{fs::File, io::{self, Read}};

fn main() {
    let r = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

     f.read_to_string(&mut s)?;

     Ok(s)
}
```

![image-20230811225659417](https://cdn.fengxianhub.top/resources-master/image-20230811225659417.png)



![image-20230811225703411](https://cdn.fengxianhub.top/resources-master/image-20230811225703411.png)



![image-20230811230036120](https://cdn.fengxianhub.top/resources-master/image-20230811230036120.png)



我们跟着老师改好之后就能自己测试效果了

![image-20230813160140149](https://cdn.fengxianhub.top/resources-master/image-20230813160140149.png)

## 3. 项目重构

现在我们想要增加一些字段并且重构一下之前的代码

![image-20230813160933784](https://cdn.fengxianhub.top/resources-master/image-20230813160933784.png)

之前的目录结构不够清晰，现在我们将项目结构进行重构

## 4. 教师管理功能





## 5. 编写服务端web前端应用

![image-20230816231601667](https://cdn.fengxianhub.top/resources-master/image-20230816231601667.png)





## 附录遇到的问题

### 0x01 表窥探技术

在执行sqlx的代码的时候遇到了报错

![image-20230816223445946](https://cdn.fengxianhub.top/resources-master/image-20230816223445946.png)



![image-20230816223504893](https://cdn.fengxianhub.top/resources-master/image-20230816223504893.png)

没想到表不存在居然连编译都通过不了，太夸张了！！！

更离谱的是我添加表后居然编译就通过了

>**看看gpt的解释**
>
>`sqlx` 在编译时进行了静态分析，并通过查询中使用的表名来检查表的存在与否。当您执行 `sqlx::query!` 宏时，它会尝试将查询字符串解析为有效的 SQL 查询，并在编译时验证查询的语法和语义。这包括检查查询中引用的表是否存在于数据库中。
>
>在您的代码中，当您将查询字符串设置为从 `course` 表中选择数据时，`sqlx` 静态分析过程会检查数据库中是否存在名为 `course` 的表。如果表存在，查询将被认为是有效的，并且代码不会报错。如果表不存在，则静态分析将检测到这个问题，并在编译时报错。
>
>这是 `sqlx` 静态分析的一个优点，它可以在编译时捕获一些潜在的问题，并提供更好的代码安全性和可靠性。通过在编译时检查表的存在与否，`sqlx` 可以避免在运行时出现相关的错误。
>
>需要注意的是，`sqlx` 的静态分析只能检查查询中引用的表是否存在，而不能检查查询的语义是否符合预期。因此，在执行查询之前，仍然需要在运行时处理数据库返回的错误，并确保查询的结果符合预期。

























