# Drips App Self-host Server
## Introduction
Drips App is a content recording app that supports article sharing, inspiration arrangement and WeChat entry, and has complete Markdown syntax support.  

All the content of Drips App is stored in the official hosting server by default. For the sake of paying attention to user privacy, we have opened our content server source to allow users to deploy themselves. The content server includes the following functions:
1. Basic content recording service
2. Article sharing direct/agent service
3. WeChat entry direct/agent service
4. File storage services (local, Tencent Cloud COS, Alibaba Cloud OSS)
At the same time, the service needs to communicate with the user server to provide the following services:
1. Authenticate the user (required)
2. Entrust the user server for article sharing and WeChat entry agent services (optional)
>The user authentication part only involves obtaining the user's unique identifier and the expiration time of the token through the token, and does not involve the exchange of other private information.
>User server refers to the official user server, which is used to provide services such as user login, registration and authorization. There is no open source plan

## Features&TODO
- [ ] Basic content recording service
- [ ] Article sharing (Direct) service
- [ ] Article sharing (proxy) service
- [ ] WeChat entry (Direct) service
- [ ] WeChat entry (proxy) service
- [ ] File storage (local)
- [ ] File storage (Tencent Cloud COS)
- [ ] File storage (AliCloud OSS)
- [ ] Docker deployment
- [ ] Tencent Cloud Function Deployment
- [ ] AliCloud function deployment

## Deployment
//TODO to be improved  

The project supports public network deployment and LAN deployment. During public network deployment, article sharing and WeChat entry services (WeChat official account needs to be configured) can be directly provided. During LAN deployment, article sharing, WeChat entry and other services related to the public network can be entrusted to the official user server agent to provide services.

This project plans to provide deployment binary packages for Docker (Docker Compose), Tencent Cloud functions, and Alibaba Cloud functions. You can also download source code to build and compile.

### **Docker deployment**
Manually or execute the following command to download the [docker compose.yml](./docker-compose.yml) file in the repo.
```Shell
mkdir DripsServer && cd DripsServer
wget  https://github.com/DripsApp/DripsServer/raw/master/docker-compose.yml
```
**Deployment**
```shell
docker-compose up -d
```
### **Tencent Cloud Function Deployment**
Go to [Releases](https://github.com/DripsApp/DripsServer/releases) Page, find the latest version of Tencent cloud compressed package, download it locally and upload it in Tencent cloud function
### **Source code construction**