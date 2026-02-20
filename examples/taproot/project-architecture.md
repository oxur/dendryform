# Taproot Architecture Document

**Version:** 1.0
**Date:** 2026-02-18
**Status:** Post-ship reference

---

## 1. System Overview

Taproot is a **Model Context Protocol (MCP) server** that provides natural language analytics capabilities over Google BigQuery, augmented by a domain knowledge engine. It serves as a bridge between Claude (via MCP) and an organisation's data warehouse, enabling the LLM to understand business terminology, navigate concept relationships, and write informed SQL queries.

### High-Level Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     Claude Desktop / MCP Client                  │
└────────────────────────────┬─────────────────────────────────────┘
                             │ MCP Protocol (stdio or HTTP+SSE)
┌────────────────────────────v─────────────────────────────────────┐
│                        Taproot Server                            │
│  ┌──────────┐  ┌───────────┐  ┌───────────┐  ┌────────────────┐  │
│  │  Auth    │  │  Tools    │  │  Server   │  │    Config      │  │
│  │Middleware│  │  Layer    │  │(rmcp 0.8) │  │  (confyg/twyg) │  │
│  └────┬─────┘  └─────┬─────┘  └─────┬─────┘  └────────────────┘  │
│       │              │              │                            │
│  ┌────v──────────────v──────────────v─────────────────────────┐  │
│  │                   Business Logic Layer                     │  │
│  │  ┌──────────────────────────────────────────────────────┐  │  │
│  │  │              Knowledge Engine                        │  │  │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐              │  │  │
│  │  │  │  Graph   │ │   FTS    │ │  Vector  │              │  │  │
│  │  │  │(petgraph)│ │(tantivy) │ │(lancedb) │              │  │  │
│  │  │  └──────────┘ └──────────┘ └──────────┘              │  │  │
│  │  │  ┌──────────┐ ┌──────────────────────┐               │  │  │
│  │  │  │  Cards   │ │  Content Provider    │               │  │  │
│  │  │  │ (loader) │ │  (InMemoryProvider)  │               │  │  │
│  │  │  └──────────┘ └──────────────────────┘               │  │  │
│  │  └──────────────────────────────────────────────────────┘  │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │  │
│  │  │   BigQuery   │  │    Redis     │  │  User Model  │      │  │
│  │  │   Client     │  │   Client     │  │   Engine     │      │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘      │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 2. Workspace Structure

```
taproot/
├── Cargo.toml                    # Workspace root (resolver = "2")
├── crates/
│   ├── taproot-server/           # Single crate — all server code
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # Binary entry point (delegates to lib::run_server)
│   │       ├── lib.rs            # Server initialisation, transport dispatch
│   │       ├── config.rs         # Config struct, env var loading (confyg)
│   │       ├── cli.rs            # CLI argument parsing (clap)
│   │       ├── deploy.rs         # Cloud Run detection, dev-only tool filtering
│   │       ├── error.rs          # Error types (TaprootError, KnowledgeError)
│   │       ├── server.rs         # MCP server struct, 25 #[tool] registrations
│   │       ├── auth/             # OAuth2 + JWT authentication
│   │       │   ├── mod.rs
│   │       │   ├── middleware.rs  # Tower auth layer for HTTP mode
│   │       │   ├── token.rs      # JWKS cache, JWT validation
│   │       │   └── metadata.rs   # RFC 9728 / RFC 8414 metadata endpoints
│   │       ├── bq/               # BigQuery integration
│   │       │   ├── mod.rs
│   │       │   ├── client.rs     # BqClient, BqOperations trait
│   │       │   ├── sql.rs        # SQL validation (sqlparser)
│   │       │   └── types.rs      # Query results, table metadata
│   │       ├── redis/            # Redis for history & cost tracking
│   │       │   ├── mod.rs
│   │       │   ├── client.rs     # RedisClient, RedisOps trait
│   │       │   ├── history.rs    # Query history storage
│   │       │   └── cost.rs       # Daily cost tracking
│   │       ├── knowledge/        # Domain knowledge engine
│   │       │   ├── mod.rs        # KnowledgeEngine facade (~313 LoC)
│   │       │   ├── types.rs      # ConceptCard, Category, Tier, CardFrontmatter
│   │       │   ├── card.rs       # Card loading from filesystem (~555 LoC)
│   │       │   ├── graph.rs      # In-memory knowledge graph (~1,117 LoC)
│   │       │   ├── fts.rs        # Full-text search engine (~707 LoC)
│   │       │   ├── vector.rs     # Vector/embedding search (~785 LoC)
│   │       │   └── provider.rs   # InMemoryProvider, ContentItemProvider
│   │       ├── tools/            # MCP tool business logic
│   │       │   ├── mod.rs        # Shared helpers (error_to_tool_result)
│   │       │   ├── knowledge.rs  # Knowledge tool handlers (~648 LoC)
│   │       │   ├── diagnostics.rs # Debug tool handlers
│   │       │   ├── schema.rs     # BQ schema tools
│   │       │   ├── query.rs      # BQ query tools
│   │       │   ├── freshness.rs  # Data freshness tools
│   │       │   ├── history.rs    # Query history tools
│   │       │   ├── cost.rs       # Cost tracking tools
│   │       │   └── user_model.rs # User model tools
│   │       └── user_model/       # Per-user learning pipeline
│   │           ├── mod.rs        # UserModelEngine
│   │           ├── types.rs      # UserConceptCard, UserProfile
│   │           ├── generator.rs  # User card generation
│   │           ├── continuity.rs # Session continuity
│   │           ├── profile.rs    # Profile synthesis
│   │           └── worker.rs     # Background processing worker
│   └── design/                   # Design documents (ODM-managed)
│       ├── docs/                 # Approved + under-review docs
│       └── dev/                  # Development guides
├── concept-cards/                # Domain content
│   └── initial-setup/            # Seed cards (10 cards)
└── workbench/                    # Working documents
```

---

## 3. Component Details

### 3.1 Server & Transport Layer

**File:** `src/server.rs` (~1,043 LoC), `src/lib.rs` (~317 LoC)

The MCP server uses **rmcp 0.8** with macro-based tool registration (`#[tool_router]` + `#[tool]`). It supports two transports:

- **stdio** — Direct Claude Desktop integration (`rmcp::transport::io::stdio()`)
- **Streamable HTTP** — Cloud Run deployment with OAuth2 auth middleware

**TaprootServer** holds all shared state:

- `config: Arc<Config>` — Immutable configuration
- `bq: Option<Arc<OnceCell<Arc<dyn BqOperations>>>>` — Lazy-init BQ client
- `redis: Option<Arc<dyn RedisOps>>` — Optional Redis connection
- `knowledge: Option<Arc<KnowledgeEngine>>` — Optional knowledge engine
- `user_model: Option<Arc<UserModelEngine>>` — Optional user model

**Tool inventory (25 total):**

| Category | Tools | Count |
|----------|-------|-------|
| Diagnostics | ping, echo, debug_config, debug_env, debug_tcp, debug_https, debug_credentials, debug_oauth2_refresh, debug_bq_connect, debug_deps | 10 |
| BigQuery | list_datasets, describe_table, get_sample_rows, explain_query, execute_query, check_data_freshness | 6 |
| Cost/History | get_query_history, get_daily_cost | 2 |
| Knowledge | get_concept, list_concepts, search_domain_knowledge, semantic_search, query_knowledge_graph | 5 |
| User Model | search_user_model, get_user_profile | 2 |

**Security model:** Additive — production router includes only safe tools; dev-only tools (echo, debug_credentials, debug_oauth2_refresh, debug_bq_connect, debug_deps) are added back only in local mode.

### 3.2 Authentication (`src/auth/`)

**Files:** `middleware.rs`, `token.rs`, `metadata.rs` (~1,223 LoC total)

- **AuthLayer** — Tower middleware for HTTP transport; validates JWT Bearer tokens
- **JwksCache** — Caches Google's JSON Web Key Sets for token verification
- **Metadata endpoints** — Serves RFC 9728 (Protected Resource Metadata) and RFC 8414 (Authorization Server Metadata) for OAuth2 discovery
- **stdio mode** — No auth; uses deterministic `{USER}@local` identity

### 3.3 BigQuery Integration (`src/bq/`)

**Files:** `client.rs`, `sql.rs`, `types.rs` (~1,478 LoC total)

- **BqClient** — Wraps `gcp-bigquery-client`, implements `BqOperations` trait
- **SQL validation** — Uses `sqlparser` to reject DDL/DML; enforces read-only SELECT
- **Cost controls** — `maximumBytesBilled` cap, row limits (default 1000)
- **Lazy init** — Client created on first use via `OnceCell` (works around Claude Desktop sandbox timing)

### 3.4 Redis Integration (`src/redis/`)

**Files:** `client.rs`, `history.rs`, `cost.rs` (~1,000 LoC total)

- **RedisClient** — Connection manager, implements `RedisOps` trait
- **Query history** — Per-user query log with SQL, results, timestamps, cost
- **Cost tracking** — Daily per-user and global byte counters with budget warnings

### 3.5 Knowledge Engine (`src/knowledge/`)

**Files:** `mod.rs`, `types.rs`, `card.rs`, `graph.rs`, `fts.rs`, `vector.rs`, `provider.rs` (~3,900 LoC total)

This is the core domain intelligence subsystem. Built once at startup from a directory of concept card markdown files.

#### 3.5.1 Domain Types (`types.rs`, 232 LoC)

```rust
pub enum Category { Metrics, Entities, Tables, Pipelines, BusinessLogic, Industry }
pub enum Tier { Foundational, Intermediate, Advanced }
pub enum ExtractionConfidence { High, Medium, Low }
pub enum AccessType { InMemory, Redis, BqSchema, File }

pub struct CardFrontmatter {
    concept, slug, category, subcategory, tier,
    source, source_slug, authors, chapter, chapter_number, section,
    access_type, access_path, extraction_confidence,
    aliases, prerequisites, extends, related, contrasts_with, answers_questions,
    user_email, query_count, last_queried,
}

pub struct ConceptCard { frontmatter, body, quick_definition }
pub struct ConceptSummary { slug, concept, category, tier, quick_definition, aliases }
pub struct ConceptFilter { category, tier, source_slug }
```

#### 3.5.2 Card Loading (`card.rs`, ~555 LoC)

- `load_cards_from_directory(path)` — Walks directory, reads markdown files, parses YAML frontmatter into `CardFrontmatter`, extracts body and quick definition
- Uses `serde_yaml` for frontmatter, `walkdir` for file discovery

#### 3.5.3 Knowledge Graph (`graph.rs`, ~1,117 LoC)

- Built on **petgraph** (`DiGraph<GraphNode, GraphEdge>`)
- Nodes created from concept cards; edges from `prerequisites`, `extends`, `related`, `contrasts_with` frontmatter fields
- **Algorithms:** related concepts, shortest path, N-hop neighbourhood, centrality ranking (degree-based), prerequisite topological sort, dependent discovery
- **Stats:** Node/edge counts, category distribution, relationship type distribution
- **Validation:** Orphan detection, self-loop detection, dangling reference detection
- **Runtime mutation:** `add_concept_card()` for user model integration

#### 3.5.4 Full-Text Search (`fts.rs`, ~707 LoC)

- Built on **Tantivy 0.22**
- Schema: `slug` (stored), `concept` (text, boost 3.0), `description` (text, boost 2.0), `body` (text, boost 1.0), `category` (string), `tier` (string), `aliases` (text, boost 1.5)
- Query: Multi-field weighted query with category/tier filters
- Returns `SearchResult { slug, concept, category, tier, score, snippet }`

#### 3.5.5 Vector Search (`vector.rs`, ~785 LoC)

- Built on **LanceDB 0.26** + **fastembed 4**
- Embedding: Local `fastembed` models (e.g., `all-MiniLM-L6-v2`)
- Text composition: `"{concept} | {category} | {aliases} | {quick_definition} | {body}"`
- Hybrid search: Reciprocal Rank Fusion (RRF) merging vector + FTS results
- Optional — gracefully degrades if embedding model unavailable

#### 3.5.6 Content Provider (`provider.rs`, ~222 LoC)

- **InMemoryProvider** — Holds `Vec<ConceptCard>` with `HashMap<String, usize>` index
- **ContentItemProvider** trait — Abstraction for list/get/filter operations

#### 3.5.7 KnowledgeEngine Facade (`mod.rs`, ~313 LoC)

```rust
pub struct KnowledgeEngine {
    provider: InMemoryProvider,
    fts: FtsEngine,
    graph: Arc<RwLock<GraphEngine>>,
    vector: Option<VectorEngine>,
}
```

Build flow: `build(cards_dir)` → load cards → build FTS index → build graph → create provider → optionally build vector engine (async, after construction)

### 3.6 User Model Engine (`src/user_model/`)

**Files:** `mod.rs`, `types.rs`, `generator.rs`, `continuity.rs`, `profile.rs`, `worker.rs` (~1,979 LoC total)

- **Per-user learning pipeline** — Generates "user concept cards" from query history
- **UserModelEngine** — Holds per-user FTS indexes and vector stores
- **Profile synthesis** — Aggregates query patterns into analytical profiles
- **Background worker** — Processes query events asynchronously via Redis pub/sub

### 3.7 Configuration (`src/config.rs`)

- Uses **confyg** for TOML config + env var overrides
- **Env var prefix:** `TAPROOT_`
- Key sections: `port`, `transport`, `bq` (project, dataset, credentials), `redis` (url), `concept_cards_path`, `lancedb` (enabled, db_path, embedding_model, cache_dir), `user_model` (enabled), `oauth` (jwks_url, client_ids), `tls` (cert/key paths), `logging` (twyg config)

### 3.8 Error Handling (`src/error.rs`)

Two error enums:

- **TaprootError** — Top-level: `Config`, `Bq`, `Redis`, `Knowledge`, `Auth`, `UserModel`
- **KnowledgeError** — Domain-specific: `CardNotFound`, `CardParse`, `Fts`, `Graph`, `Vector`

Tools convert errors to MCP `CallToolResult` with `is_error: true` via `error_to_tool_result()`.

### 3.9 Deployment (`src/deploy.rs`)

- **Cloud Run detection** — Checks `K_SERVICE` env var
- **Dev-only tools list** — `["echo", "debug_credentials", "debug_oauth2_refresh", "debug_bq_connect", "debug_deps"]`
- Additive security model ensures prod never exposes dev tools

---

## 4. Startup Sequence

```
1. Config::load()              — Load TOML + env vars
2. twyg::setup()               — Initialise structured logging
3. config.validate()           — Fail on invalid values
4. deploy::is_cloud_run()      — Detect deployment context
5. BQ OnceCell                 — Prepare lazy-init cell (if configured)
6. RedisClient::new()          — Connect to Redis (if configured)
7. KnowledgeEngine::build()    — Load cards, build FTS, build graph
8. VectorEngine::build_async() — Embed & index vectors (if enabled)
9. UserModelEngine::new()      — Create user model (needs knowledge + redis)
10. spawn_worker()             — Background user model processing
11. TaprootServer::new()       — Register tools, apply security model
12. run_stdio() / run_http()   — Start transport
```

---

## 5. Data Flow

### Query Execution Flow

```
MCP Client → execute_query tool
  → SQL validation (sqlparser — reject DDL/DML)
  → Cost estimation (dry run)
  → BigQuery execution
  → Redis: store history + update cost counters
  → Return: rows, metadata, cost, truncation info
```

### Knowledge Query Flow

```
MCP Client → search_domain_knowledge tool
  → FtsEngine.search(query, category, tier, limit)
  → Tantivy: multi-field weighted query
  → Return: ranked SearchResult[]

MCP Client → semantic_search tool (mode=hybrid)
  → FTS search (keyword results)
  → Vector search (embedding similarity)
  → Reciprocal Rank Fusion merge
  → Return: combined ranked results

MCP Client → query_knowledge_graph (operation=get_prerequisites)
  → GraphEngine: topological sort from target node
  → Return: ordered prerequisite chain
```

---

## 6. Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| rmcp | 0.8 | MCP protocol (macro-based tools) |
| axum | 0.8 | HTTP server framework |
| tokio | 1 | Async runtime |
| petgraph | 0.6 | In-memory directed graph |
| tantivy | 0.22 | Full-text search engine |
| lancedb | 0.26 | Vector database |
| fastembed | 4 | Local embedding generation |
| gcp-bigquery-client | 0.22 | BigQuery API |
| redis | 1.0 | Redis client |
| confyg | 0.3 | Config loading (TOML + env) |
| twyg | 0.6 | Structured logging |
| serde_yaml | 0.9 | YAML frontmatter parsing |
| sqlparser | 0.53 | SQL validation |
| jsonwebtoken | 9 | JWT verification |

---

## 7. Lines of Code Summary

| Module | Files | Approx. LoC |
|--------|-------|-------------|
| Knowledge Engine | 7 | ~3,900 |
| User Model | 6 | ~1,979 |
| Server + Tools | 10 | ~2,700 |
| BigQuery | 4 | ~1,478 |
| Auth | 4 | ~1,223 |
| Redis | 4 | ~1,000 |
| Config/CLI/Deploy/Error | 4 | ~800 |
| **Total** | **39** | **~13,080** |

---

## 8. Planned Refactor

The knowledge engine (~3,900 LoC) plus knowledge tools (~648 LoC) and config handlers (~200 LoC) are being migrated to use the **fabryk crate ecosystem** (`~/lab/oxur/ecl/crates/fabryk*`). This will:

- Delete ~3,457 lines (graph.rs, fts.rs, vector.rs, tools/knowledge.rs, config handlers)
- Replace with fabryk backends via adapter pattern (GraphExtractor, DocumentExtractor, VectorExtractor)
- Upgrade rmcp 0.8 → 0.14 (macro-based → trait-based tool registration)
- Use fabryk-mcp-* for tool registries

See design doc: `crates/design/docs/02-under-review/0005-taproot-server-fabryk-migration-plan.md`
