# 点点App 自托管服务器
[English](./README.md) | [简体中文](./README_zh-CN.md)
## 简介
点点App是一款支持文章分享、灵感整理、微信录入功能的内容记录App，具有完整的Markdown语法支持  
点点App 的所有内容由默认存放于官方的托管服务器，出于对用户隐私的重视与我们开源了我们的内容服务器部分，用以允许用户自部署。该内容服务器包含以下功能：  
1. 基础的内容记录服务
2. 文章分享直接/代理服务
3. 微信录入直接/代理服务
4. 文件存储服务（本地、腾讯云COS、阿里云OSS）

同时，该服务需要与用户服务器进行通信以提供以下服务：  
1. 对用户进行鉴权（必选）
2. 委托用户服务器进行文章分享、微信录入代理服务（可选）

> 其中对用户鉴权部分仅涉及通过token获取用户唯一标识符与token过期时间，不涉及其他隐私信息的交换。  

> 用户服务器指官方用户服务器，用于提供用户登录注册授权等服务，无开源计划

## 特性 & TODO
- [ ] 基础的内容记录服务
- [ ] 文章分享(Direct)服务
- [ ] 文章分享(Proxy)服务
- [ ] 微信录入(Direct)服务
- [ ] 微信录入(Proxy)服务
- [ ] 文件存储(本地)
- [ ] 文件存储(腾讯云COS)
- [ ] 文件存储(阿里云OSS)
- [ ] Docker 部署
- [ ] 腾讯云函数部署
- [ ] 阿里云函数部署

## 部署
// TODO 待完善

本项目支持公网部署与局域网部署，公网部署时可直接提供文章分享与<u>微信录入服务(需配置微信公众号)</u>，局域网部署时文章分享与微信录入等涉及公网的服务均可委托官方用户服务器代理提供服务。  

本项目计划提供Docker(Docker Compose)、腾讯云函数、阿里云函数的部署二进制包，您也可以自行下载源代码进行构建编译。

### **Docker 部署**
手动或执行以下命令下载仓库内 [docker-compose.yml](./docker-compose.yml) 文件
```Shell
mkdir DripsServer && cd DripsServer
wget https://github.com/DripsApp/DripsServer/raw/master/docker-compose.yml
```
**部署**
```shell
docker-compose up -d
```

### **腾讯云函数部署**
前往[Releases](https://github.com/DripsApp/DripsServer/releases)页面，找到最新版本的腾讯云压缩包，下载到本地后在腾讯云函数上传

### **源代码构建**
