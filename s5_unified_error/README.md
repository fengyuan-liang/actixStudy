# actix学习笔记

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


### 2.4 数据库CRUD

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






