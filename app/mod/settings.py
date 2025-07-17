# import os
import sys

from box import Box
from loguru import logger as LOG

# from collections import namedtuple


LOG.remove()
LOG.add(
    sys.stdout,
    format="{time:YYMMDD HH:mm:ss.SSS} | PID:{process} | {level: <8} | {module}:{function}:{line} |> {message}",  # 自定义格式 1640442
    level="INFO",  # 全局日志级别[^1]
    colorize=True,
    enqueue=True,  # 异步写入文件
)
# LOG.add(sys.stdout, level="INFO")
# LOG.add(sys.stdout, level="WARNING")
# LOG.add(sys.stdout, level="ERROR")
# LOG.add(sys.stdout, level="CRITICAL")
# 添加文件输出（可选）
# LOG.add(
#    "app.log",  # 日志文件路径
#    rotation="10 MB",  # 自动轮转
#    retention="30 days",  # 保留期限
#    compression="zip",  # 压缩格式
#    level="DEBUG",  # 文件记录更详细级别
# )

CONF = {
    "project": "l3streamlit",
    "version": "25.7.17.1542",
    "author": "Zoom.Quiet",
    "feedback": "zquiet+fmhub@gmail.com",
    "license": "MIT@2025",
    #   debugging...
    "DEBUG": 1,
    #   data
    "toml": "/opt/src/streamlit/l3/docs/FM_STORY.toml",
}

CFG = Box(CONF)
VERSION = f"{CFG.project}:v{CFG.version}"
