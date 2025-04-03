### 启动密钥管理服务
1. 确保您的项目已经编译
2. 创建必要的数据目录
3. 运行示例程序
以下是具体的操作步骤：
### 1.编译项目
```sh
cd password_manager
cargo build --release
```

### 2.创建数据目录
```sh
mkdir -p ./ data
```

### 3. 运行密钥管理服务
```sh
cargo run --release --example key_management_db_example
```
或者，如果您想直接运行编译后的二进制文件：
```sh
./target/release/examples/key_management_db_example
```
### 4. 验证服务是否正常运行
```plaintext
密钥管理插件（数据库版）已启动
密钥创建成功: {...}
数据加密成功
数据解密成功: 这是一些需要持久化存储的敏感数据
密钥轮换成功: {...}
密钥列表: [...]
审计日志: [...]
按 Ctrl+C 停止插件...
```
