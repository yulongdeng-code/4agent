# NexusAgent 架构设计文档 v1.0

## 核心概念：四大开源项目的完美融合

本系统融合四个开源智能体项目的核心优势：

| 项目 | 核心能力 | 在本系统中的角色 |
|------|---------|------------------|
| **OpenClaw** | 工具生态系统 & MCP 协议 | 工具层：17000+ 工具接入 |
| **OpenManus** | Planning-Execution-Verification 闭环 | 核心引擎：端到端任务交付 |
| **OpenHuman** | 分层记忆树 & Token 压缩 | 知识层：智能上下文管理 |
| **HermesAgent** | 自我进化 & 遗传算法 | 学习层：系统自我优化 |

---

## 系统架构

### 五层架构设计

```
┌─────────────────────────────────────────────────┐
│  Layer 5: 涌现层 (Emergence)                    │
│  自组织行为、模式识别、创新能力                  │
│  ├─ 自我进化引擎                               │
│  ├─ Skill 自动生成                             │
│  ├─ Prompt 优化                                │
│  └─ 知识总结                                   │
├─────────────────────────────────────────────────┤
│  Layer 4: 共振层 (Resonance)                    │
│  多工具协调、自适应调度                         │
│  ├─ 工具编排                                   │
│  ├─ 并发执行                                   │
│  ├─ 负载均衡                                   │
│  └─ 失败恢复                                   │
├─────────────────────────────────────────────────┤
│  Layer 3: 协议层 (Protocol)                     │
│  PEV 三层闭环、任务状态机                       │
│  ├─ Planning Engine                           │
│  ├─ Execution Engine                          │
│  ├─ Verification Engine                       │
│  └─ DAG 管理                                   │
├─���───────────────────────────────────────────────┤
│  Layer 2: 记忆层 (Memory)                       │
│  分层记忆树、Token 压缩                         │
│  ├─ Memory Tree                               │
│  ├─ Token Compressor                          │
│  ├─ Context Builder                           │
│  └─ Retrieval Engine                          │
├─────────────────────────────────────────────────┤
│  Layer 1: 基础层 (Foundation)                   │
│  LLM 接口、工具注册表、数据存储                 │
│  ├─ LLM Provider                              │
│  ├─ Tool Registry                             │
│  ├─ Data Store (SQLite)                       │
│  └─ Config Manager                            │
└─────────────────────────────────────────────────┘
```

---

## 核心模块设计

### Module 1: Memory Tree（第1-2周）

**目标**：实现分层记忆系统，支持6种检索方式

**核心功能**：
```
✅ 分层记忆存储
✅ 6 种检索方式：
   - Entity Search (实体搜索)
   - Topic Query (主题查询)
   - Time-based Query (时间查询)
   - Source Query (来源查询)
   - Semantic Search (语义搜索)
   - Drill-down (深度下钻)
✅ Token 压缩
✅ 上下文生成
✅ SQLite 存储
```

**性能指标**：
- 搜索延迟: < 50ms
- 压缩率: > 60%
- 存储容量: 百万级记忆

**文件结构**：
```
src/core/memory/
├── mod.rs           # 模块导出
├── tree.rs          # 核心树结构
├── node.rs          # 节点定义
├── store.rs         # SQLite 存储
├── retrieval.rs     # 检索引擎
└── tests/           # 单元测试
```

---

### Module 2: PEV Engine（第3-4周）

**目标**：实现完整的 Planning-Execution-Verification 闭环

**三层流程**：

#### Planning 层
```
需求 → 理解 → 分解为子任务 → 建立依赖图 → 生成执行计划
```

#### Execution 层
```
执行计划 → 创建沙箱 → 生成代码 → 执行代码 → 调用工具 → 返回结果
```

#### Verification 层
```
结果 → 验证格式 → 验证完整性 → 验证正确性 → 生成反��
        ↓ 失败
    自动修复 → 重新执行
```

**验收标准**：
- 端到端成功率: ≥ 95%
- 自动修复率: ≥ 80%
- PEV 周期: < 5s (简单任务)

---

### Module 3: Tool Registry（第5周）

**目标**：实现工具生态系统，支持多种工具类型

**支持的工具类型**：
```
1. 内置工具 (Built-in)
   - 代码执行
   - 文件操作
   - 网络请求

2. 插件工具 (Plugin)
   - 动态加载
   - 生命周期管理

3. MCP 工具 (Model Context Protocol)
   - 标准化接口
   - 17000+ 工具适配

4. 自定义工具 (Custom)
   - Python 函数
   - Shell 脚本
   - HTTP 端点
```

---

### Module 4: TokenJuice Compressor（第6周）

**目标**：实现智能 Token 压缩，平均压缩率 > 60%

**压缩管道**：
```
Raw Text (10,000 tokens)
  ↓ HTML→Markdown
Step 1 (8,000 tokens)
  ↓ URL 缩短
Step 2 (7,500 tokens)
  ↓ 去重
Step 3 (6,000 tokens)
  ↓ 关键词提取
Compressed (3,000 tokens) ← 70% 节省
```

**效果指标**：
- 压缩率: 60-80%
- 信息保留率: 90-95%
- 处理延迟: < 100ms

---

### Module 5: Evolution Engine（第7-8周）

**目标**：实现系统自我进化，自动生成 Skills 和优化 Prompts

**进化流程**：
```
执行成功案例 (Traces)
    ↓
特征提取 (Feature Extraction)
    ├─ 行为模式
    ├─ 工具使用模式
    ├─ 错误处理模式
    └─ 性能特征
    ↓
Skill 生成 (Skill Generation)
    ├─ 可复用任务模板
    ├─ 最佳实践
    └─ 工具链模式
    ↓
Prompt 进化 (Prompt Evolution)
    ├─ GEPA 算法
    ├─ Pareto 优化
    └─ 多目标优化
    ↓
下次执行 ← 性能提升 30-50%
```

**子模块**：
- **Skill Generator** (Python): 从执行轨迹自动生成可复用 Skill
- **GEPA Algorithm** (Python): 遗传算法优化 Prompts
- **Prompt Optimizer** (Rust): 集成和调度

---

## 完整的数据流

### 用户请求的完整处理过程

```
📝 用户请求
"帮我写一个快速排序算法，要求 O(n log n)"
        ↓
🧠 记忆层
  从 Memory Tree 获取上下文：
  - 过去的排序实现
  - 用户的编码风格
  - 类似的成功案例
        ↓
📋 规划层
  分解为子任务：
  1. 算法设计
  2. 代码实现
  3. 测试编写
  4. 性能验证
  建立依赖关系 DAG
        ↓
⚙️  执行层
  并行执行独立任务：
  - Task 1: 生成算法代码
    → LLM 调用 (经 TokenJuice 压缩)
    → Docker 沙箱编译
  - Task 2: 生成测试
    → 调用测试生成工具
  - Task 3: 性能分析
    → 调用性能工具
        ↓
✅ 验证层
  多维度验证：
  ✓ 代码编译成功
  ✓ 所有测试通过
  ✓ 时间复杂度是 O(n log n)
  ✓ 文档完整
        ↓
📦 交付
  完整的交付物：
  - 源代码
  - 测试文件
  - 性能报告
  - 文档
        ↓
🔄 进化
  记录执行轨迹 → 评分 → 提取 Skill → 优化 Prompt
  下次类似请求自动提升 30-50%
```

---

## 开发路线图

### Phase 1: Foundation (Week 1-2)

**目标**: Memory Tree 完整实现

```
Day 1-2: 数据结构设计 + 数据库建表
Day 3-4: CRUD 操作 + 6 种检索
Day 5:   Token 压缩集成
Day 6-7: 测试 + 文档

验收标准:
✅ 1000 条记忆插入成功
✅ 搜索延迟 < 50ms
✅ 压缩率 > 60%
✅ 所有测试通过
```

---

### Phase 2: Core Engine (Week 3-4)

**目标**: PEV 三层闭环完整工作

```
Day 1-2: Planning Engine
Day 3:   Execution Engine
Day 4:   Verification Engine
Day 5-6: Docker 沙箱集成
Day 7:   完整 end-to-end 测试

验收标准:
✅ 端到端成功率 ≥ 95%
✅ 自动修复率 ≥ 80%
✅ 所有测试通过
```

---

### Phase 3: Tool Ecosystem (Week 5-6)

**目标**: 完整的工具系统 + Token 压缩

```
Week 5: Tool Registry 实现
Week 6: TokenJuice 集成

验收标准:
✅ 工具自动注册和发现
✅ 压缩率 > 60%
✅ 所有工具调用成功
```

---

### Phase 4: Self-Evolution (Week 7-8)

**目标**: 系统自我进化完整工作

```
Day 1-2: Skill Generator (Python)
Day 3-4: GEPA Algorithm (Python)
Day 5:   Rust ↔ Python IPC
Day 6-7: 完整集成测试

验收标准:
✅ 自动生成 Skill
✅ Prompt 进化生效
✅ 性能提升 30-50%
```

---

### Phase 5: Production (Week 9-10)

**目标**: 完整系统可用于生产

```
Week 9: 性能优化 + 完整文档
Week 10: Docker 配置 + CI/CD + 部署脚本

验收标准:
✅ 所有 100+ 测试通过
✅ 完整的文档和示例
✅ 一键部署脚本工作
✅ 性能指标达成
```

---

## 性能目标

| 指标 | 目标 | 说明 |
|------|------|------|
| 内存占用 | < 500MB | 空闲时 |
| 记忆查询 | < 50ms | P95 |
| 工具调用 | < 500ms | 包括网络 |
| Token 压缩 | 60-80% | 压缩率 |
| PEV 周期 | < 5s | 简单任务 |
| 端到端成功 | ≥ 95% | 简单任务 |
| 自动修复 | ≥ 80% | 修复成功 |
| 系统可用性 | 99.9% | 月度 |

---

## 技术栈

### 核心语言
- **Rust 1.75+**: 主程序、性能关键路径
- **Python 3.11+**: 进化算法、AI 计算
- **TypeScript 5.4+**: UI (可选)

### 关键库
- **tokio**: 异步运行时
- **sqlx/rusqlite**: 数据库
- **serde**: 序列化
- **reqwest**: HTTP 客户端
- **dspy**: Prompt 优化 (Python)

### 基础设施
- **SQLite**: 默认数据库
- **Docker**: 沙箱执行环境
- **GitHub Actions**: CI/CD

---

## 验收标准总览

```
✅ Week 2 完成: Memory Tree 完整工作
   - 100+ 单元测试全部通过
   - 搜索延迟 < 50ms
   - 压缩率 > 60%

✅ Week 4 完成: PEV 引擎完整工作
   - 端到端成功率 ≥ 95%
   - 自动修复率 ≥ 80%
   - 50+ 集成测试通过

✅ Week 6 完成: 工具系统 + 压缩
   - 工具自动注册
   - Token 压缩生效

✅ Week 8 完成: 进化系统工作
   - 自动生成 Skill
   - Prompt 进化生效

✅ Week 10 完成: 生产就绪
   - 所有 100+ 测试通过
   - 完整文档
   - 一键部署脚本
   - 性能基准达成
```

---

## 下一步行动

1. ✅ **架构规划** (本文档)
2. 📝 **Module 1 实现**: Memory Tree
3. 📝 **Module 2 实现**: PEV Engine
4. 📝 **Module 3 实现**: Tool Registry
5. 📝 **Module 4 实现**: TokenJuice
6. 📝 **Module 5 实现**: Evolution Engine
7. 📝 **集成和优化**
8. 📝 **部署和文档**

---

**文档版本**: 1.0
**最后更新**: 2026-05-21
**维护者**: NexusAgent Team
