<div align="center">
<img src="https://upload-bbs.miyoushe.com/upload/2025/05/08/365152535/0a154b759159adf6beb79d1582528fae_4082085489423633137.png" width=200 />

# meme_emoji_rust

<p align="center">
  <img src="https://img.shields.io/github/license/MemeCrafters/meme-generator-rs">
  <a href="https://crates.io/crates/meme_generator">
    <img src="https://img.shields.io/crates/v/meme_generator">
  </a>
  <a href="https://pypi.org/project/meme-generator">
    <img src="https://img.shields.io/pypi/v/meme-generator">
  </a>
  <a href="https://qm.qq.com/q/DVb9aGPmaQ">
    <img src="https://img.shields.io/badge/QQ%E7%BE%A4-743103809-orange">
  </a>
</p>

</div>

## 表情包扩展仓库 meme_emoji_rust

🚀 基于 [meme-generator-rs](https://github.com/MemeCrafters/meme-generator-rs) 做的扩展表情包仓库

*✨* 为你的聊天机器人添加更多趣味表情生成！


## 特性

- ✅ **海量表情** 偶尔做做热门表情包，也欢迎投稿高清有趣的素材
- ⚡ **实时生成** 支持通过指令快速生成表情
- 🔄 **搭配使用** 需要搭配 [meme-generator-rs](https://github.com/MemeCrafters/meme-generator-rs) 一起使用

- ✨**搭配演示架构图所示(仅供参考)：**
![架构图](./picture/meme_emoji_rs.jpg)



## 已实现表情示例
### 参考预览图：
<img src="./picture/Phone.png" alt="image-20250312190444844" style="zoom:50%;" />

### 表情包清单

```
1. aircraft_cup_air_play (空气玩法)
2. aircraft_cup_cleaning_liquid (清洗液)
3. aircraft_cup_commemorative_edition_saint_sister (纪念版圣修女)
4. aircraft_cup_hoshino_alice (拉拉队偶像)
5. aircraft_cup_idol_heartbeat (偶像心跳)
6. aircraft_cup_jissbon (杰士邦)
7. aircraft_cup_limited_edition_saint_sister (纯洁臀)
8. aircraft_cup_liuli_zi (琉璃子)
9. aircraft_cup_pure_buttocks (纯洁臀)
10. aircraft_cup_saint_sister (圣修女)
11. aircraft_cup_selena (魔女之森)
12. aircraft_cup_summer_liuli_zi (夏日琉璃子)
13. aircraft_cup_taimanin_asgi (对魔忍)
14. all_the_days (一生一世)
15. atri_like (亚托莉喜欢)
16. begged_me (求我)
17. congyu_dislike (丛雨讨厌)
18. deer_help (帮鹿/帮🦌)
19. deer_se (🦌/鹿)
20. dinosaur_head (恐龙头)
21. dog_face (🐶)
22. fbi_photo (fbi/FBI)
23. hitachi_mako_together (和她在一起)
24. ice_tea_head (冰红茶)
25. ikun_durian_head (榴莲坤头)
26. ikun_head (小黑子)
27. kfc_head (KFC/kfc)
28. kun_like (坤坤喜欢)
29. kurogames_mp (鸣批/鸣P/鸣p/鸣潮玩家/鸣潮男)
30. kurogames_songlun (松伦哥指/潮批)
31. mahiro_fuck (真寻中指/中指/🖕🏻)
32. mi_monkey (米猴/🐒/🐵)
33. mihoyo_funina_death_penalty (死刑)
34. mihoyo_funina_round_head (芙芙圆形头像)
35. mihoyo_funina_square_head (芙芙方形头像)
36. mihoyo_genshin_impact_op (OP/op/Op/oP)
37. mihoyo_genshin_impact_players (原批/原神玩家)
38. miss_in_my_sleep (睡梦中想念)
39. murasame_blackboard (丛雨黑板)
40. murasame_husband (丛雨老公)
41. murasame_like (丛雨喜欢)
42. s_ninja (S忍/s忍)
43. spend_christmas (一起圣诞)
44. swimsuit_group_photo (泳衣合影)
45. together_two (在一起)
46. torture_yourself (折磨自己)
47. xinxi_news (新喜报)
48. youzi_kitchen (柚子厨)
49. youzi_question_mark (震惊柚子厨)
```



## 配置信息

### 仓库文件信息

| Name                 | Attribute | Info                         |
| :------------------- | --------- | ---------------------------- |
| picture              | folder    | 说明文档图片                 |
| resources            | folder    | 表情包素材                   |
| src                  | folder    | source code                  |
| Cargo.lock           | file      | 锁定依赖的具体版本           |
| Cargo.toml           | file      | 声明依赖的版本范围           |
| LICENSE              | file      | 许可文件                     |
| meme_emoji_rust.xlsx | file      | 工作进度；普通用户可忽略     |
| paint_rsinfo.ps1     | file      | 工作脚本；普通用户可忽略     |
| README.md            | file      | 说明文档                     |
| rsinfo.txt           | file      | 工作脚本数据；普通用户可忽略 |

### Windows Config:
> 文件参考路径 C:\Users\anyliew\.meme_generator\config.toml
```bash
[meme]
load_builtin_memes = true # 是否加载内置表情包
load_external_memes = true  # 是否加载外部表情包
#true false
meme_disabled_list = []  # 禁用的表情包列表，填写表情的 `key`

[resource]
resource_url = "https://cdn.jsdelivr.net/gh/MemeCrafters/meme-generator-rs@"  # 下载内置表情包图片/字体时的资源链接
download_fonts = true  # 是否下载字体

[font]
use_local_fonts = true  # 是否使用本地文件夹下的字体
default_font_families = ["Noto Sans SC", "Noto Color Emoji"]  # 默认字体

[encoder]
gif_max_frames = 200  # 限制生成的 gif 帧数
gif_encode_speed = 1  # gif 编码速度，范围为 1 ~ 30，数字越大，编码速度越快，但图片质量越差

[api]
baidu_trans_appid = ""  # 百度翻译api相关，部分表情需要使用
baidu_trans_apikey = ""  # 可在 百度翻译开放平台 (http://api.fanyi.baidu.com) 申请

[server]
host = "0.0.0.0"  # web server 监听地址
port = 2233  # web server 端口
```


## 相关链接

- [meme-generator-rs](https://github.com/MemeCrafters/meme-generator-rs) 

- meme-generator-contrib-rs 额外表情仓库[MemeCrafters/meme-generator-contrib-rs](https://github.com/MemeCrafters/meme-generator-contrib-rs) 

## 反馈
单个表情有问题反馈请附带日志和截图

答复受限网络没那么快及时处理

> issues https://github.com/anyliew/meme_emoji_rust/issues 


## 声明

本仓库的表情素材等均来自网络，如有侵权请联系作者删除
