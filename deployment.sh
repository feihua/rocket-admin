#!/bin/bash

cargo build -r

#停止服务
docker stop rocket-admin


#删除容器
docker rm rocket-admin

#删除镜像
docker rmi rocket-admin:v1

#删除none镜像
docker rmi $(docker images | grep "none" | awk '{print $3}')

#构建服务
docker build -t rocket-admin:v1 -f Dockerfile .

#启动服务
docker run -itd --net=host --name=rocket-admin rocket-admin:v1
