# RSCMS（Rust内容管理系统）

---

[English](./README.md) | [中文](./README_cn.md)

**介绍：**

RSCMS是一个采用Rust全新开发的开源内容管理系统，旨在为开发人员提供一个强大高效的平台的数字内容管理系统。利用Rust的力量和性能，RSCMS提供了一个现代化解决方案，用于创建、组织和传递各种应用程序和网站的内容。

**关键特点：**

1. **基于Rust的性能：** RSCMS利用Rust的速度和可靠性，提供高性能的内容管理功能，确保最佳效率和可扩展性。

2. **模块化架构：** 具有灵活和模块化的架构，开发人员可以轻松扩展和定制RSCMS，以满足其特定的内容管理需求。

3. **用户友好界面：** RSCMS提供直观用户友好的界面，用于内容创建、编辑和发布，使用户能够有效管理其数字内容。

4. **安全性和可靠性：** 设计时考虑安全性，RSCMS优先考虑数据保护和系统完整性，为管理敏感内容提供安全环境。

5. **社区驱动开发：** RSCMS是一个开源项目，欢迎社区贡献来增强功能，修复错误并推动持续改进。

**快速开始：**

环境要求：
- Rust（最新稳定版）
- Docker和Docker Compose
- Git

按照以下步骤设置和运行RSCMS：

1. **克隆代码仓库：**
   ```bash
   git clone https://github.com/rscms-dev/rscms.git
   cd rscms
   ```

2. **配置环境变量：**
   在项目根目录创建 `.env` 文件，包含以下变量：
   ```env
   # 服务器配置
   SERVER_HOST=127.0.0.1    # 服务器主机地址
   SERVER_PORT=3000         # 服务器端口号

   # 数据库配置
   DATABASE_URL=mysql://root:example@localhost:3306/rscms

   # JWT配置（用于身份验证）
   JWT_SECRET=your-super-secret-and-ultra-long-secret-key

   # 邮件配置（可选）
   SMTP_HOST=smtp.example.com
   SMTP_PORT=587
   SMTP_USERNAME=your-username
   SMTP_PASSWORD=your-password
   SMTP_FROM_EMAIL=noreply@example.com
   ```

3. **启动开发数据库：**
   ```bash
   cd db
   docker-compose up -d
   ```
   这将启动：
   - MySQL数据库（可通过 localhost:3306 访问）
   - PHPMyAdmin界面（可通过 <http://localhost:8080> 访问）
   
   默认数据库凭据：
   - 用户名：root
   - 密码：rscms
   - 数据库：rscms

4. **构建和运行RSCMS：**
   ```bash
   cargo build
   cargo run
   ```

5. **访问API：**
   API将在 `http://{SERVER_HOST}:{SERVER_PORT}` 上可用
   
   注意：RSCMS目前仅提供API接口，暂无Web界面。

**开发数据库管理：**
- PHPMyAdmin可通过http://localhost:8080访问
- 停止数据库：`docker-compose down`
- 查看数据库日志：`docker-compose logs`
- 数据库数据持久化存储在`./db/data`目录
- 初始化迁移文件自动从`./db/migrations`目录加载

**参与其中：**

加入我们，与Rust一起共同塑造内容管理的未来！无论您是希望贡献代码的开发人员，希望改善用户体验的设计师，还是寻找现代CMS解决方案的内容创作者，每个人都有机会参与并在RSCMS项目中发挥作用。

**立即探索RSCMS：**

访问我们的GitHub组织"rscms-dev"，获取最新的代码库，为项目做出贡献，并加入一个充满激情的开发人员社区，共同致力于在Rust中实现现代内容管理。
