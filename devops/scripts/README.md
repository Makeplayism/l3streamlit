> 配置 systemd 来管理运行在 mamba 虚拟环境中的 Streamlit 应用。

## 创建 systemd 服务文件

创建文件 `/etc/systemd/system/streamlit-l3.service`：

```ini
[Unit]
Description=Streamlit L3 FMHub Application
After=network.target

[Service]
Type=simple
User=zoomq
Group=zoomq
WorkingDirectory=/opt/src/streamlit/l3

# 设置环境变量
Environment="PATH=/home/zoomq/miniforge3/envs/py313/bin:/usr/local/bin:/usr/bin:/bin"
Environment="CONDA_PREFIX=/home/zoomq/miniforge3/envs/py313"
Environment="CONDA_DEFAULT_ENV=py313"
Environment="STREAMLIT_BROWSER_GATHER_USAGE_STATS=false"

# 启动命令 - 使用 mamba 环境中的 python
ExecStart=/home/zoomq/miniforge3/envs/py313/bin/python -m streamlit run app/main.py \
    --server.port=18051 \
    --server.address=127.0.0.1 \
    --server.headless=true \
    --browser.serverAddress=localhost \
    --browser.gatherUsageStats=false

# 重启策略
Restart=always
RestartSec=10

# 日志
StandardOutput=journal
StandardError=journal

# 资源限制（可选）
# MemoryLimit=2G
# CPUQuota=80%

[Install]
WantedBy=multi-user.target
```

## 或者使用 mamba run 方式

如果你想明确使用 mamba 来激活环境，可以这样配置：

```ini
[Unit]
Description=Streamlit L3 FMHub Application
After=network.target

[Service]
Type=simple
User=zoomq
Group=zoomq
WorkingDirectory=/opt/src/streamlit/l3

# 初始化 mamba
Environment="MAMBA_ROOT_PREFIX=/home/zoomq/miniforge3"
Environment="MAMBA_EXE=/home/zoomq/miniforge3/bin/mamba"
Environment="STREAMLIT_BROWSER_GATHER_USAGE_STATS=false"

# 使用 mamba run 来执行
ExecStart=/home/zoomq/miniforge3/bin/mamba run -n py313 \
    streamlit run app/main.py \
    --server.port=18051 \
    --server.address=127.0.0.1 \
    --server.headless=true

Restart=always
RestartSec=10

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

## 使用 shell 脚本包装（推荐）

创建启动脚本 `/opt/src/streamlit/l3/start-streamlit.sh`：

```bash
#!/bin/bash
# 初始化 conda/mamba
source /home/zoomq/miniforge3/etc/profile.d/conda.sh
source /home/zoomq/miniforge3/etc/profile.d/mamba.sh

# 激活环境
mamba activate py313

# 设置工作目录
cd /opt/src/streamlit/l3

# 启动 Streamlit
exec streamlit run app/main.py \
    --server.port=18051 \
    --server.address=127.0.0.1 \
    --server.headless=true \
    --browser.serverAddress=localhost \
    --browser.gatherUsageStats=false
```

设置执行权限：
```bash
chmod +x /opt/src/streamlit/l3/start-streamlit.sh
```

然后创建简化的 systemd 服务：

```ini
[Unit]
Description=Streamlit L3 FMHub Application
After=network.target

[Service]
Type=simple
User=zoomq
Group=zoomq
WorkingDirectory=/opt/src/streamlit/l3
ExecStart=/opt/src/streamlit/l3/start-streamlit.sh
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

## 管理服务

```bash
# 重新加载 systemd 配置
sudo systemctl daemon-reload

# 启用服务（开机自启）
sudo systemctl enable streamlit-l3.service

# 启动服务
sudo systemctl start streamlit-l3.service

# 查看服务状态
sudo systemctl status streamlit-l3.service

# 查看日志
sudo journalctl -u streamlit-l3.service -f

# 重启服务
sudo systemctl restart streamlit-l3.service

# 停止服务
sudo systemctl stop streamlit-l3.service
```

## 日志管理

创建专门的日志配置 `/etc/systemd/system/streamlit-l3.service.d/override.conf`：

```ini
[Service]
StandardOutput=append:/var/log/streamlit/l3.log
StandardError=append:/var/log/streamlit/l3-error.log
```

创建日志目录：
```bash
sudo mkdir -p /var/log/streamlit
sudo chown zoomq:zoomq /var/log/streamlit
```

## 配置日志轮转

创建 `/etc/logrotate.d/streamlit-l3`：

```
/var/log/streamlit/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 0644 zoomq zoomq
    postrotate
        systemctl reload streamlit-l3.service > /dev/null 2>&1 || true
    endscript
}
```

## 监控和告警（可选）

创建健康检查脚本 `/opt/src/streamlit/l3/healthcheck.sh`：

```bash
#!/bin/bash
curl -f http://localhost:18051/_stcore/health || exit 1
```

添加到 systemd 服务中：
```ini
[Service]
# ... 其他配置
ExecStartPost=/bin/sleep 10
ExecStartPost=/opt/src/streamlit/l3/healthcheck.sh
```

这样配置后，你的 Streamlit 应用就能在 mamba 环境中稳定运行，并享受 systemd 提供的进程管理、日志记录和自动重启等功能。