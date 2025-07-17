# from mod.fmhub import NPC

# from mod.picker import TOMLCfg as TCFG
import time
from typing import Dict, List, Optional

import toml
from mod.settings import CFG, LOG

import streamlit as st

st.set_page_config(
    page_title="[L3]未来之门",
    page_icon="🚪",
    # layout="wide",
    # initial_sidebar_state="expanded",
)


# 加载故事数据
@st.cache_data
def load_story_data():
    """加载TOML格式的故事数据"""
    # 这里假设FM_STORY.toml文件在同一目录下
    # 实际使用时需要提供正确的路径
    try:
        with open(CFG.toml, "r", encoding="utf-8") as f:
            return toml.load(f)
    except FileNotFoundError:
        # 如果文件不存在，返回一个示例数据结构
        st.error("未找到故事文件 FM_STORY.toml")
        return None


def stream_text_smart(text):
    """智能分块流式输出，兼顾速度和效果"""
    import re

    # 按标点符号和空格分割
    chunks = re.findall(r"[^，。！？；：\s]+[，。！？；：\s]*", text)
    for chunk in chunks:
        yield chunk
        time.sleep(0.18)  # 调整速度


# 故事导航类
class StoryNavigator:
    def __init__(self, story_data: Dict):
        self.story_data = story_data
        self.fm_choice = story_data.get("FM_CHOICE", {})
        self.fm_story = story_data.get("FM_STORY", {})

    def get_choice_story(self, level: int) -> Optional[Dict]:
        """获取指定层级的选择故事"""
        key = f"FM_CHOICE.{level}"
        return self.fm_choice.get(str(level), None)

    def get_story_branch(self, path: str) -> Optional[Dict]:
        """根据路径获取故事分支"""
        return self.fm_story.get(path, None)

    def get_all_possible_paths(self, max_depth: int = 6) -> List[str]:
        """获取所有可能的故事路径"""
        paths = []

        def generate_paths(current_path: str, depth: int):
            if depth > max_depth:
                return

            for choice in ["R", "B"]:
                new_path = current_path + choice
                if new_path in self.fm_story:
                    paths.append(new_path)
                    generate_paths(new_path, depth + 1)

        generate_paths("", 1)
        return sorted(paths)


# 主应用函数
def main():
    # 初始化session state
    if "choice_path" not in st.session_state:
        st.session_state.choice_path = ""
    if "current_level" not in st.session_state:
        st.session_state.current_level = 1

    # 加载故事数据
    story_data = load_story_data()
    # if not story_data:
    #    # 使用示例数据进行演示
    #    story_data = create_sample_data()

    navigator = StoryNavigator(story_data)

    # 标题
    st.title("🚪 未来之门 - L3")
    st.divider()

    # 显示起始故事
    col1, col2 = st.columns(2)

    with col1:
        st.subheader("📖 选择的起点")
        fm_choice = navigator.fm_choice.get("FM_CHOICE", {})
        if fm_choice:
            st.markdown(f"**{fm_choice.get('title', '')}**")
            # st.markdown(fm_choice.get("story", ""))
            st.write_stream(stream_text_smart(fm_choice.get("story", "")))

    with col2:
        st.subheader("🌟 故事的开端")
        fm_story = navigator.fm_story.get("FM_STORY", {})
        if fm_story:
            st.markdown(f"**{fm_story.get('title', '')}**")
            # st.markdown(fm_story.get("story", ""))
            st.write_stream(stream_text_smart(fm_story.get("story", "")))

    st.divider()

    # 显示当前路径的故事
    if st.session_state.choice_path:
        st.subheader(f"📍 你的选择路径: {st.session_state.choice_path}")

        # 显示当前路径的故事
        current_story = navigator.get_story_branch(
            st.session_state.choice_path
        )
        if current_story:
            st.info(f"**{current_story.get('title', '')}**")
            # st.markdown(current_story.get("story", ""))
            st.write_stream(stream_text_smart(current_story.get("story", "")))
            st.divider()

    # 显示当前层级的选择
    if st.session_state.current_level <= 6:  # 假设最多6层
        choice_story = navigator.get_choice_story(
            st.session_state.current_level
        )
        if choice_story:
            st.subheader(f"🔮 第 {st.session_state.current_level} 层选择")
            st.markdown(f"**{choice_story.get('title', '')}**")
            # st.markdown(choice_story.get("story", ""))
            st.write_stream(stream_text_smart(choice_story.get("story", "")))

            # 选择按钮
            st.divider()
            choice = st.radio(
                "请选择我们的未来:",
                ["Red", "Blue"],
                captions=[
                    choice_story.get("red", "红色: 未来的挑战"),
                    choice_story.get("blue", "蓝色: 未来的机遇"),
                    # "红色: 未来的挑战",
                    # "蓝色: 未来的机遇",
                ],
                horizontal=True,
                key=f"choice_{st.session_state.current_level}",
            )

            # 确认选择按钮
            if st.button("确认选择", type="primary"):
                # 更新路径
                if choice == "Red":
                    st.session_state.choice_path += "R"
                else:
                    st.session_state.choice_path += "B"

                # 更新层级
                st.session_state.current_level += 1

                # 强制重新运行以更新显示
                st.rerun()

    else:
        st.success("🎯 你已经完成了所有决择！")
        st.balloons()

        # 显示 FM_NOEND 内容
        fm_noend = story_data.get("FM_NOEND", {})
        if fm_noend:
            st.divider()
            st.subheader("🌌 终章")
            st.markdown(f"**{fm_noend.get('title', '')}**")
            # st.markdown(fm_noend.get("story", ""))
            st.write_stream(stream_text_smart(fm_noend.get("story", "")))

    # 侧边栏功能
    with st.sidebar:
        st.header("🎮 控制面板")

        if st.button("🔄 重新开始"):
            st.session_state.choice_path = ""
            st.session_state.current_level = 1
            st.rerun()

        st.divider()

        # 显示所有可能的路径
        if st.checkbox("显示所有可能路径"):
            all_paths = navigator.get_all_possible_paths()
            st.write(f"共有 {len(all_paths)} 条可能的故事线:")
            for path in all_paths[:20]:  # 只显示前20条
                st.code(path)
            if len(all_paths) > 20:
                st.write(f"... 还有 {len(all_paths) - 20} 条路径")


if __name__ == "__main__":
    main()
