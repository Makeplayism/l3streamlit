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