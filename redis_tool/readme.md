# redis_tool 简价

redis_tool 是一个模拟redis-cli的工具,对redis查询出来的结果进行格式化的展示,采用rust进行开发,可以多平台下面编译使用：

- **跨平台**： 可以在多平台下编译，跨平台使用；

- **零学习成本**：完全兼容redis命令,也可以做为redis命令的学习工具；

- **互动 Console**: 通过命令行 console。 


### 登录
**cmd**: 
```bash
    redis_tool <host> <port> [password]
```
 
**Case**:
```bash
  root~# redis_tool.exe 127.0.0.1 6379 "password"
    .-"""-.
    / .===. \
    \/ 6 6 \/
    ( \___/ )
 _________ooo__\_____/_____________
/                                  \
|    Connect Redis Success!         |
\_______________________ooo________/
     |  |  |
     |_ | _|
     |  |  |
     |__|__|
     /-'Y'-\
    (__/ \__)
127.0.0.1:6379~[db0]#>
```
**Keys**
```bash
### exists 命令:判断Key是否存在
127.0.0.1:6379~[db0]#> exists cache_key
true

### type 命令：获取key类型
127.0.0.1:6379~[db0]#> type cache_key
+-----------+--------+
| key       | type   |
+-----------+--------+
| cache_key | string |
+-----------+--------+
```

