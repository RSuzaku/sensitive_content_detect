# 敏感词检测服务

这是一个敏感词检测的一个服务，基于给定的字典数据，以及停用词，用 DFA 算法，进行敏感词检测

运行环境

> rustc 1.67.1
>
> cargo 1.67.1

启动命令

```shell
cargo run --bin sensitive_detect_server
```

API 接口

```shell
curl --location 'http://127.0.0.1:10925/sensitive' \
--header 'Content-Type: application/json' \
--data '{"text":"小日子在排核污水"}'
```

输出如下

```json
{
  "code": 0,
  "msg": "success",
  "data": [
    { "category": "政治", "content": "小日子" },
    { "category": "政治", "content": "核污水" }
  ]
}
```



如有问题，请联系wechat: 415401944
