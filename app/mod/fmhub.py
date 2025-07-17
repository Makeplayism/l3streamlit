import streamlit as st

from .picker import Picker
from .settings import CFG, LOG


def NPC():
    """
    NPC: Non-Player Character
    """
    genre = st.radio(
        "What's your favorite movie genre",
        [":rainbow[Comedy]", "***Drama***", "Documentary :movie_camera:"],
        captions=[
            "Laugh out loud.",
            "Get the popcorn.",
            "Never stop learning.",
        ],
    )

    if genre == ":rainbow[Comedy]":
        st.write("You selected comedy.")
    else:
        st.write("You didn't select comedy.")
