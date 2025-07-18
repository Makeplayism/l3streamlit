#!/bin/bash
set -e

# 设置环境
export PATH="/home/zoomq/miniforge3/envs/py313/bin:$PATH"
export HOME="/home/zoomq"

# 进入工作目录
cd /opt/src/streamlit/l3

# 启动 streamlit
exec /home/zoomq/miniforge3/envs/py313/bin/streamlit run app/main.py \
    --server.port=18051 \
    --server.address=127.0.0.1 \
    --server.headless=true




