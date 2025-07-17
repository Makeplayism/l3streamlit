import tomllib


class TOMLCfg:
    def __init__(self, config_file):
        with open(config_file, "rb") as f:
            self.data = tomllib.load(f)

    def get(self, key_path, default=None):
        """
        使用点号分隔的路径获取值
        例如: config.get("FM_CHOICE.1.title")
        """
        keys = key_path.split(".")
        value = self.data

        for key in keys:
            if isinstance(value, dict) and key in value:
                value = value[key]
            else:
                return default

        return value

    def get_all_choices(self, key="FM_CHOICE"):
        """获取所有选择项"""
        fm_choice = self.data.get(key, {})
        choices = []

        # 提取主信息
        main_info = {
            "title": fm_choice.get("title"),
            "story": fm_choice.get("story"),
        }

        # 提取所有数字键的子选项
        for key, value in fm_choice.items():
            if key.isdigit() and isinstance(value, dict):
                choices.append(
                    {
                        "id": int(key),
                        "title": value.get("title"),
                        "story": value.get("story"),
                    }
                )

        return main_info, sorted(choices, key=lambda x: x["id"])


if __name__ == "__main__":

    # 使用示例
    config = TOMLCfg("config.toml")

    # 获取特定值
    title = config.get("FM_CHOICE.1.title")
    print(f"选项1的标题: {title}")

    # 获取所有选择
    main_info, choices = config.get_all_choices()
    print(f"\n主标题: {main_info['title']}")
    for choice in choices:
        print(f"\n选项 {choice['id']}: {choice['title']}")
