构建镜像：docker build -t article-recorder .
启动容器：
bash
docker run -d --name my-recorder -p 3000:3000 -v $(pwd)/.env:/app/.env article-recorder
说明：映射 
.env
 文件以配置环境，并将容器 3000 端口映射至宿主机。

 项目运行依赖

 运行命令。

 