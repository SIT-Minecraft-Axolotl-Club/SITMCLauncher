<div align="center">
  <img src="./apps/app/icons/128x128.png" width="128" height="128" alt="SITMC 启动器" />
  <h1>SITMC 启动器</h1>
  <p><strong>上海应用技术大学 Minecraft 六角恐龙社的官方启动器</strong></p>

  <p>
    <a href="https://www.sitmc.club">官方网站</a> ｜
    <a href="https://cloud.sitmc.club/s/7kaFe">下载启动器</a> ｜
    <a href="https://skin.sitmc.club">用户中心</a> ｜
    <a href="https://www.sitmc.club/guide">游玩指南</a> ｜
    <a href="https://qm.qq.com/q/YTb3tqvyYE">加入交流群</a>
  </p>
</div>

---

**SITMC 启动器**（SITMCL）是社团基于开源项目 [Modrinth App](https://github.com/modrinth/code) 二次开发的 Minecraft Java 版启动器，免费、开源、无广告。它把「注册账号 → 装好整合包 → 进服」压缩成一条流程：登录社团账号后，社团服务器发布的实例会自动安装并在版本更新时自动同步，不需要玩家自己去翻整合包、对齐版本。

目前只提供 **Windows 10 / 11（x64）** 版本。

## 快速开始

1. **下载**：[点此下载 SITMC 启动器](https://cloud.sitmc.club/s/7kaFe)
2. **安装**：双击运行安装包。Windows Defender 可能提示「已保护你的电脑」，点「更多信息」→「仍要运行」即可。
3. **登录**：启动后点「使用 SIT-Minecraft 账户登录」，浏览器会打开授权页面。校内同学可直接用 **Eduroam**（学号 + 信息门户 OA 密码）登录。
4. **创建角色**：首次登录会提示创建角色，这里填的就是你的游戏 ID（英文、数字、下划线）。
5. **进服**：在首页选择要玩的实例，点「Play」。缺失的游戏文件和整合包会自动下载。

还没有账号？请先阅读[隐私政策](https://www.sitmc.club/privacy)，再到[用户中心](https://skin.sitmc.club)注册。
校外玩家可以通过 [MUA 用户中心](https://skin.mualliance.ltd/) 使用教育邮箱等方式完成认证。

## 它做了什么

| 能力 | 说明 |
| --- | --- |
| **社团实例托管** | 实例由服务器统一发布，包含整合包、游戏版本与加载器。点一下就能装好，服务器更新版本时客户端会跟着更新，不用手动重装。 |
| **社团账号登录** | 内置社团用户中心登录（浏览器授权），支持 Eduroam / OA 认证；登录状态会自动续期，长时间挂机也不会掉线。 |
| **服务器在线人数** | 首页的每个实例会显示所连服务器的实时在线人数（如「在线 3/2026」），每分钟刷新，不用进游戏也能看服务器是否有人。 |
| **皮肤与披风** | 内置社团皮肤站，可浏览、上传、切换皮肤与披风，游戏内同步生效。 |
| **友站实例** | 可以在「友站」页浏览兄弟社团发布的实例，同样支持一键安装与启动。 |
| **内置工具** | 种子地图、投影工坊、资源包与光影管理、存档管理、日志分析等。 |

## 社团服务器

| 服务器 | 地址 | 说明 |
| --- | --- | --- |
| 生存世界 | `play.sitmc.club` | 社团主服务器 |
| 像素上应 | `creative.sitmc.club` | 建筑与创作服 |
| 跑酷世界 | `game.sitmc.club` | 小游戏与跑酷 |
| 模组世界 | `mod.sitmc.club` | 模组服 |
| Bingo | `bingo.sitmc.club` | 活动玩法 |

服务器状态与在线人数也可以在[像素上应在线地图](https://www.sitmc.club/pixelsit)查看。

## 常见问题

**Q：提示「Java 21 无法安装」或安装实例失败？**
启动器会自动下载 Minecraft 官方 Java 运行时。这一步依赖网络，若连接 Mojang 服务器失败，它会短暂重试后回滚，避免留下装了一半的实例。换个网络环境重试即可；反复失败请把「安装报告」发到交流群。

**Q：启动器会自己更新吗？**
会。更新包使用签名校验，下载完成后自动安装，不需要手动覆盖。也可以在「设置 → 更新」里改成延迟一天获取。

**Q：需要自己装 Modrinth 或改成别的账号系统吗？**
不需要。这个版本固定连接社团账号系统，不需要也不能切换到其他认证源。

**Q：macOS / Linux 能用吗？**
目前不提供。社团服务器相关流程只在 Windows 版本上验证过。

**Q：以前的「美西螈启动器」还要留着吗？**
不需要。从旧版升级后，启动器会自动把实例、设置和账号迁移到新的数据目录，并在迁移完成后卸载旧版；旧版数据不会被删除。

## 关于这个项目

- 仓库：[SIT-Minecraft-Axolotl-Club/Axolotl](https://github.com/SIT-Minecraft-Axolotl-Club/Axolotl)
- 上游项目：[Modrinth App](https://github.com/modrinth/code)、[Axolotl Launcher](https://github.com/Mystic-Stars/Axolotl)
- 许可：[GPL-3.0](COPYING.md)
- 反馈问题：欢迎到[交流群 623697015](https://qm.qq.com/q/YTb3tqvyYE)反馈，或提交 [Issue](https://github.com/SIT-Minecraft-Axolotl-Club/Axolotl/issues)
- 参与开发：本地开发环境与打包发布流程见 [CONTRIBUTING.md](CONTRIBUTING.md)；社区行为规范见 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

## 相关链接

- 社团站点：[www.sitmc.club](https://www.sitmc.club) ｜ [近期动态](https://www.sitmc.club/news) ｜ [关于我们](https://www.sitmc.club/about)
- 用户中心：[skin.sitmc.club](https://skin.sitmc.club)
- 文件云盘：[cloud.sitmc.club](https://cloud.sitmc.club)
- 生存卫星：[survival.sitmc.club](https://survival.sitmc.club)
- B 站：[space.bilibili.com/9888184](https://space.bilibili.com/9888184)；抖音：[社团主页](https://v.douyin.com/Yd84UYBOnes/)
- 友情社团：[Minecraft 高校联盟](https://www.mualliance.cn/)、[燕山大学 Minecraft 社](https://ysumc.net/)、[上海交通大学 Minecraft 社](https://mc.sjtu.cn/)、[同济大学 Minecraft 社](https://www.tongji-mc.cn/)、[浙江大学 Minecraft 社](https://www.mczju.com/)、[西南科技大学 Minecraft 社](https://www.swustmc.cn/)、[西安邮电大学邮立方](https://cop.cooo.site/)

---

<div align="center">

上海应用技术大学 Minecraft 六角恐龙社 版权所有  
Copyright 2023-2026 SIT-Minecraft

本启动器是调用 Modrinth 公开 API 的独立客户端，与 Rinth, Inc. 无任何关联，也与客户端项目 Axolotl Client 无关。  
"Minecraft" 以及 "我的世界" 为美国微软公司的商标，本项目与微软公司没有从属关系。

</div>
