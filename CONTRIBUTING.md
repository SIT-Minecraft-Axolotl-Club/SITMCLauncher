# 贡献指南 (Contributing)

感谢你对 SITMC 启动器及其相关内容感兴趣！在提交代码前，请先阅读以下指南。

我们希望所有参与者都能在友善、包容的环境中协作。请先阅读[行为准则 (CODE_OF_CONDUCT.md)](CODE_OF_CONDUCT.md)，了解社区期望和问题报告方式。

## 本地开发

### 环境要求

- **Node.js**：以 [`.nvmrc`](.nvmrc) 为准
- **pnpm**：以根目录 [`package.json`](package.json) 的 `packageManager` 为准
- **Rust**：以 [`rust-toolchain.toml`](rust-toolchain.toml) 为准
- [Tauri v2 系统依赖](https://v2.tauri.app/start/prerequisites/)

### 启动开发环境

1. 初始化 Git 子模块（cubiomes 是构建期的编译依赖，缺失会导致 Rust 编译失败），启用 Corepack 并安装依赖：
   ```powershell
   git submodule update --init --recursive
   corepack enable
   pnpm install --frozen-lockfile
   ```
2. 启动开发服务器：
   ```powershell
   pnpm app:dev
   ```

### 常用检查命令

在提交代码前，建议运行以下命令确保代码符合规范：

```powershell
# 品牌名和文本合规检查
pnpm axolotl:brand-guard
# 国际化词条检查
pnpm axolotl:i18n-check
# 前端格式化及 lint
pnpm prepr:frontend:app
# Rust 格式化检查
cargo fmt --all --check
# Rust 基础检查
cargo check --package theseus_gui --features updater
```

### 构建缓存与磁盘空间

Rust 编译产物位于 `target` 目录，首次完整构建可能占用数 GB 空间。Turbo 仅缓存前端输出，不会缓存 `target/**`。桌面应用的 Tauri 构建任务已明确关闭 Turbo 缓存。

如需释放本地开发缓存，可以删除以下目录：

```powershell
Remove-Item -Recurse -Force .turbo\cache
Remove-Item -Recurse -Force target\debug
```

此操作不会删除 `target\installer-test` 中单独生成的安装包。下次启动开发模式时需要重新编译 Rust 依赖。

### 应用数据目录与数据库迁移

启动器把设置、数据库、日志与缓存放在同一个数据目录下，Windows 上是 `%APPDATA%\club.sitmc.launcher`，其中按更新通道分为 `release\app.db` 与 `beta\app.db`。

**本地构建与已安装的正式版共用这份数据目录。** 本地跑一次桌面应用，改动的就是正在使用的那份数据库，因此：

- 迁移是单向的，也不要手工改 `_sqlx_migrations` 表。
- 某个构建写入新迁移后，比它旧的构建（包括已安装的正式版）将因 sqlx 拒绝打开而无法启动，报错形如 `migration <版本号> was previously applied but is missing in the resolved migrations`。
- 需要回退时用下面的降级脚本，不要手工处理。

隔离数据目录有两种方式，推荐在开发时始终使用其中一种：

```powershell
# 构建期：该构建写入 club.sitmc.launcher-<后缀>，与正式版互不影响
$env:AXOLOTL_DATA_DIR_SUFFIX = "pr538"
pnpm app:build

# 运行时：把整个数据目录搬到指定路径（便携模式也用这个变量）
$env:THESEUS_CONFIG_DIR = "$PWD\.dev-data"
pnpm app:dev
```

如果数据库已经被较新的构建升级，用降级脚本回退（**运行前必须关闭启动器**；默认只做演练，执行时会先写一份备份）：

```powershell
# 列出已应用的迁移
node scripts/axolotl/downgrade-app-db.mjs --list

# 演练：显示将移除哪些迁移记录、删除哪些列
node scripts/axolotl/downgrade-app-db.mjs --to 20260903120000

# 实际执行
node scripts/axolotl/downgrade-app-db.mjs --to 20260903120000 --apply
```

`--to` 是**阈值而非单条**：它会移除**该版本及其之后**的全部迁移记录。请填你确认可以回退到的最老版本。

脚本默认在正式的数据目录里找库；如果那个库属于带后缀的构建，用 `--suffix` 指过去：

```powershell
node scripts/axolotl/downgrade-app-db.mjs --suffix pr538 --to 20260903120000 --apply
```

恢复备份的方法：关闭启动器，删除 `app.db` 及其 `-wal`/`-shm` 边车文件，再把 `app.db.before-downgrade-<时间戳>` 改名回 `app.db`。

脚本的几条硬性约束，遇到时请照提示处理而不是绕过：

- 只认识登记在案的迁移结构。遇到未登记的迁移会**拒绝执行**，因为删掉记录却留下它建的列/表，会让重新安装该构建时因对象重复而失败；确认无碍时用 `--allow-unmapped` 显式放行。
- 若某个已登记迁移对应的列在库里不存在，说明登记的表结构与数据库不符（版本号被复用、列被改名等），脚本同样拒绝执行。
- 新增**列**的迁移时，请在 `scripts/axolotl/downgrade-app-db.mjs` 的 `REVERTIBLE_COLUMNS` 里登记它建的每一列，否则越过它的降级会被拒绝。只做数据增删（DELETE/UPDATE）的迁移也要登记一个空数组——它没有列可删，但登记了才不会挡住降级。新增**表**的迁移目前没有登记机制，只能手工处理。

该脚本依赖 Node 内置的 `node:sqlite`，并要求 Windows（解析默认数据目录需要 `APPDATA`；其他平台请用 `--db` 指定路径）。

## 仓库范围

本项目的产品改动主要位于：

- `apps/app-frontend`
- `apps/app`
- `packages/app-lib`
- 上述包所需的共享 UI 与资源包

本仓库**不包含** Modrinth 网站、Labrinth API 或其运营服务源码。桌面端保留对 Modrinth 公共 API 的客户端兼容；如果需要参考上游实现，请仅手动挑选与本项目功能相关的改动，避免直接合并无关代码。

## 发布新版本

版本号以 Git 标签为准，必须符合[语义化版本格式](https://semver.org/lang/zh-CN/)：

```powershell
git tag -a v1.2.3 -m "SITMC Launcher 1.2.3"
git push origin v1.2.3
```

预发布版本请使用带后缀的标签（如 `v1.2.3-beta.1`）。

社团使用**自己的更新服务与签名密钥**发布，安装包在本地构建并签名后再上传，完整步骤（写更新说明、构建、上传后台、自测）见 [`docs/launcher-update-guide.md`](docs/launcher-update-guide.md)。打包前请确认：

1. 版本号已同步到 `apps/app-frontend/package.json` 与两个 `Cargo.toml`（可用 `node scripts/axolotl/set-version.mjs v<版本>`）；
2. `apps/app-frontend/src/announcements/catalog.ts` 里有对应版本的公告条目；
3. 构建产出了 `*.exe.sig`——**没有签名更新包，已安装的客户端不会收到这次更新**。

> 自动更新的公钥已固化在客户端里，签名私钥只保存在发版者本机与社团的密码管理器中，**任何时候都不要提交到仓库**。密钥的生成与轮换方式见更新指南。
