## 使用seaorm

安装脚手架

```shell
$ rust install sea-orm-cli
```

使用脚手架生成对应的实体类（也可以不使用直接改之前的代码）

```shell
$ sea-orm-cli generate entity --database-url postgres://postgres:123456@localhost:5432/tutorial --output-dir src/database
Connecting to Postgres ...
Discovering schema ...
... discovered.
Generating course.rs
    > Column `id`: i32, auto_increment, not_null
    > Column `teacher_id`: i32, not_null
    > Column `name`: String, not_null
    > Column `time`: Option<DateTime>
    > Column `description`: Option<String>
    > Column `format`: Option<String>
    > Column `structure`: Option<String>
    > Column `duration`: Option<String>
    > Column `price`: Option<i32>
    > Column `language`: Option<String>
    > Column `level`: Option<String>
Generating teacher.rs
    > Column `id`: i32, auto_increment, not_null
    > Column `name`: String, not_null
    > Column `picture_url`: String, not_null
    > Column `profile`: Option<String>
Writing src/database\course.rs
Writing src/database\teacher.rs
Writing src/database\mod.rs
Writing src/database\prelude.rs
... Done.
```

然后改一改就可以用了😁

![image-20230822234942195](https://cdn.fengxianhub.top/resources-master/image-20230822234942195.png)



