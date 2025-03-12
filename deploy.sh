#!/bin/bash

 script
# 该函数用于构建并部署Rocket-admin应用，具体步骤如下：
# 1. 使用Cargo构建Rust项目的发布版本。
# 2. 停止并移除现有的Rocket-admin容器。
# 3. 删除现有的Rocket-admin镜像。
# 4. 清理所有标记为"none"的Docker镜像。
# 5. 使用Dockerfile构建新的Rocket-admin镜像。
# 6. 运行新的Rocket-admin容器，并配置网络为host模式。

# 使用Cargo构建Rust项目的发布版本
cargo build -r

# 停止并移除现有的Rocket-admin容器
docker stop rocket-admin
docker rm -f rocket-admin

# 删除现有的Rocket-admin镜像
docker rmi -f rocket-admin:v1

# 清理所有标记为"none"的Docker镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

# 使用Dockerfile构建新的Rocket-admin镜像
docker build -t rocket-admin:v1 -f Dockerfile .

# 运行新的Rocket-admin容器，并配置网络为host模式
docker run -itd --net=host --name=rocket-admin rocket-admin:v1

