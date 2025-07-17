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
    layout="wide",
    # initial_sidebar_state="expanded",
    initial_sidebar_state="collapsed",  # 默认折叠侧边栏
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
        key = f"{level}"
        return self.fm_choice.get(key, None)
        _key = f"FM_CHOICE.{level}"
        LOG.info(f"获取选择故事: {_key}")
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


# 改进的ASCII树生成器（更美观的版本）
def generate_ascii_tree_v2(current_path: str, max_depth: int = 4) -> str:
    """生成更美观的ASCII故事树"""
    lines = []

    # 简化版本，显示更清晰
    indent = "      "
    lines.append("         ┌─R─R─R...")
    lines.append("       ┌─R┤")
    lines.append("       │ └─B─R...")
    lines.append("FMHub─┤")
    lines.append("       │ ┌─R...")
    lines.append("       └─B┤")
    lines.append("         └─B─B...")

    # 标记当前路径
    if current_path:
        # 根据路径重新生成带标记的树
        lines = []

        def add_branch(prefix, path, depth, is_last):
            if depth > max_depth:
                return

            connector = "└─" if is_last else "├─"

            # 检查是否在当前路径上
            is_on_path = current_path.startswith(path) if path else True
            marker = (
                "+" if is_on_path and len(path) == len(current_path) else ""
            )

            if path:
                choice = path[-1]
                display = f"{marker}{choice}"

                # 如果还有后续路径，添加省略号
                if depth == max_depth and len(current_path) > depth:
                    display += "..."

                lines.append(f"{prefix}{connector}{display}")

            if depth < max_depth:
                # 添加子分支
                extension = "  " if is_last else "│ "
                new_prefix = prefix + extension if path else ""

                add_branch(new_prefix, path + "R", depth + 1, False)
                add_branch(new_prefix, path + "B", depth + 1, True)

        lines.append("FMHub")
        add_branch("", "", 0, False)

        lines.append("")
        lines.append(f"当前选择: {current_path} (标记为 +)")

    return "\n".join(lines)


st.markdown(
    """
<style>
.stRadio > div > label:nth-child(1) {
    background: linear-gradient(45deg, #ff6b6b, #ff8e8e);
    color: white;
    border-radius: 8px;
    padding: 8px 16px;
    font-weight: bold;
}

.stRadio > div > label:nth-child(2) {
    background: linear-gradient(45deg, #4dabf7, #74c0fc);
    color: white;
    border-radius: 8px;
    padding: 8px 16px;
    font-weight: bold;
}
</style>
""",
    unsafe_allow_html=True,
)


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
    st.title("🚪 未来之门 - Life 3.0")
    st.divider()

    # 显示起始故事
    col1, col2 = st.columns(2)

    with col1:
        st.subheader("📖 选择的起点")
        # fm_choice = navigator.fm_choice.get("FM_CHOICE", {})
        fm_choice_0 = navigator.fm_choice.get("0", {})
        if fm_choice_0:
            st.markdown(f"**{fm_choice_0.get('title', '')}**")
            st.markdown(fm_choice_0.get("story", ""))
            # st.write_stream(stream_text_smart(fm_choice_0.get("story", "")))
            st.markdown("""注意, 每次决择未来, 只有**两种姿态:**""")
            # st.write_stream(stream_text_smart(fm_choice_0.get("red", "")))
            # st.write_stream(stream_text_smart(fm_choice_0.get("blue", "")))
            st.markdown(fm_choice_0.get("red", ""))
            st.markdown(fm_choice_0.get("blue", ""))

    with col2:
        st.subheader("🌟 故事的开端")
        # fm_story = navigator.fm_story.get("FM_STORY", {})
        fm_story = story_data.get("FM_START", {})
        if fm_story:
            st.markdown(f"**{fm_story.get('title', '')}**")
            st.markdown(fm_story.get("story", ""))
            # st.write_stream(stream_text_smart(fm_story.get("story", "")))

    st.divider()

    # 显示当前路径的故事
    if st.session_state.choice_path:
        st.subheader(f"📍 你的路径决择: {st.session_state.choice_path}")

        # 显示当前路径的故事
        current_story = navigator.get_story_branch(
            st.session_state.choice_path
        )
        if current_story:
            st.info(f"**{current_story.get('title', '')}**")
            st.markdown(current_story.get("story", ""))
            # st.write_stream(stream_text_smart(current_story.get("story", "")))
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

            # ASCII艺术路径图
            st.subheader("🌳 故事树")
            ascii_tree = generate_ascii_tree_v2(
                st.session_state.choice_path, max_depth=4
            )
            st.code(ascii_tree, language="text")

            # 路径列表
            st.subheader("📝 路径列表")
            for path in all_paths[:20]:  # 只显示前20条
                if path == st.session_state.choice_path:
                    st.success(f"▶ {path} (当前路径)")
                else:
                    st.code(path)
            if len(all_paths) > 20:
                st.write(f"... 还有 {len(all_paths) - 20} 条路径")


if __name__ == "__main__":
    main()
