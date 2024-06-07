# Geektime Rust 语言训练营

第五周：构建高性能互联网应用

生成测试密钥对

```sh
cd chat_server
openssl genpkey -algorithm ed25519 -out fixtures/private.pem
openssl pkey -in fixtures/private.pem -pubout -out fixtures/decoding.pem
```
