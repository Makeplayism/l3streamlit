# from mod.fmhub import NPC

from mod.picker import TOMLCfg as TCFG
from mod.settings import CFG, LOG

import streamlit as st

st.set_page_config(
    page_title="[L3]未来之门",
    page_icon="🔮",
    # layout="wide",
    # initial_sidebar_state="expanded",
)


def main():
    st.title("FMHub: 生命3.0 未来简史")

    fm = TCFG(CFG.toml)
    main_info, choices = fm.get_all_choices()
    st.subheader(main_info["title"])
    st.write(main_info["story"])
    for choice in choices:
        st.markdown(f"### 选项 {choice['id']}: {choice['title']}")
        st.write(choice["story"])

    genre = st.radio(
        "嗯哼, 选择你的未来:",
        ["Red", "Blue"],
        captions=[
            "红色: 未来的挑战",
            "兰色: 未来的机遇",
        ],
        horizontal=True,
    )

    if genre == "Red":
        st.write("You selected Red: 未来的挑战.")
    else:
        st.write("You selected Blue: 未来的机遇.")


if __name__ == "__main__":
    main()
