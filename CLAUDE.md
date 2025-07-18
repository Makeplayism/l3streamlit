# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **l3streamlit** - a Chinese interactive story-based game built with Streamlit, inspired by the "Life 3.0" book. The game presents players with binary choices about humanity's future relationship with AI, creating branching narratives based on their decisions.

## Architecture

### Core Components

- **Main Application**: `/app/main.py` - Entry point containing the Streamlit UI and game logic
- **Story Navigator**: `StoryNavigator` class handles story data parsing and path navigation
- **Configuration**: `/app/mod/settings.py` - Centralized configuration using python-box
- **Story Data**: `/docs/FM_STORY.toml` - TOML-based story content and choice definitions
- **Modules**:
  - `fmhub.py` - NPC interaction components (minimal implementation)
  - `picker.py` - TOML configuration parser with dot-notation access

### Data Structure

The game uses a hierarchical choice system:
- **6 levels** of binary choices (Red/Blue)
- **126 possible story combinations** (2^6 paths)
- Each choice path generates unique story content
- Stories are stored in TOML format with nested sections

### Key Features

- **Interactive Story Tree**: ASCII visualization of choice paths
- **Session State Management**: Tracks player progress through `st.session_state`
- **Streaming Text**: Smart text streaming for dramatic effect
- **Path Visualization**: Shows current position in story tree
- **Multiple Endings**: Based on accumulated choices

## Development Commands

### Running the Application
```bash
# Install dependencies
pip install -r requirements.txt

# Run the Streamlit app
streamlit run app/main.py
```

### Development Setup
```bash
# The app expects story data at the configured path
# Default: /opt/src/streamlit/l3/docs/FM_STORY.toml

# Key configuration in app/mod/settings.py:
# - Project version: 25.7.17.1542
# - Story data path: CFG.toml
# - Logging with loguru
```

## Configuration Files

- **requirements.txt**: Python dependencies (streamlit, python-box, loguru, toml)
- **docs/FM_STORY.toml**: Story content with sections:
  - `FM_CHOICE.0-6`: Choice prompts for each level
  - `FM_STORY.*`: Story branches for each path combination
  - `FM_START`: Initial story setup
  - `FM_NOEND`: Final ending content

## Story Content Structure

Story paths are encoded as strings of 'R' (Red) and 'B' (Blue) choices:
- `"R"` - First level red choice
- `"RB"` - Red then blue choice
- `"RBRBRB"` - Complete 6-level path

Each path in FM_STORY contains:
- `title`: Chapter title
- `story`: Narrative content (supports markdown)

## Technical Notes

- Uses `@st.cache_data` for story data loading
- Session state manages: `choice_path`, `current_level`
- Custom CSS styling for choice buttons (red/blue gradients)
- Supports both English and Chinese content
- Logging configured with custom format and colors

## Development Environment

- **Python 3.x** with Streamlit framework
- **TOML** for configuration and story data
- **Loguru** for structured logging
- **python-box** for configuration management
- **DevOps**: Nginx configuration and deployment scripts in `/devops/`

## Project Status

Current version: 25.7.17.1542 (as of July 2025)
- Working MVP with full story tree implementation
- All 126 story combinations covered
- Interactive UI with session management
- Ready for production deployment