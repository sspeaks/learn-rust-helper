# Quest 34: Archive Capstone

**🎮 Quest:** The mission is complete—almost. The final step is to fetch a batch of records from the remote archive network, then persist them to the local SQLite store. Two worlds collide: the async HTTP skills from World 4 and the synchronous database skills from World 6. But there's a catch: `rusqlite::Connection` cannot cross an `async` boundary. Design your solution to respect that constraint.

## Objective

Implement three functions forming a staged async/sync pipeline:
- `fetch_archive_batch`: asynchronously fetch and decode remote records
- `persist_archive_batch`: synchronously persist an already-fetched batch
- `load_archive_preview`: synchronously read back a deterministic preview

This capstone synthesizes async HTTP, serde, and SQLite in one cohesive workflow.

## Public API

```rust
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RemoteArchiveRecord {
    pub mission_code: String,
    pub artifact: String,
    pub priority: u8,
}

pub struct ArchiveBatch {
    pub fetched_at: String,
    pub records: Vec<RemoteArchiveRecord>,
}

pub struct PersistReport {
    pub inserted: usize,
    pub skipped: usize,
}

pub enum ArchiveCapstoneError {
    Request(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
    Decode(reqwest::Error),
    Sql(rusqlite::Error),
}

/// Stage 1 (async): fetch and decode remote records.
pub async fn fetch_archive_batch(
    base_url: &str,
    mission_code: &str,
) -> Result<ArchiveBatch, ArchiveCapstoneError>

/// Stage 2 (sync): persist an already-fetched batch.
/// Keep database writes synchronous so `&Connection` never crosses an `.await` boundary.
pub fn persist_archive_batch(
    conn: &Connection,
    batch: &ArchiveBatch,
) -> Result<PersistReport, ArchiveCapstoneError>

pub fn load_archive_preview(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<RemoteArchiveRecord>, ArchiveCapstoneError>
```

## Behavioral Rules

### `fetch_archive_batch`
1. Send a GET request to `{base_url}/archive/{mission_code}`.
2. A network failure returns `ArchiveCapstoneError::Request`.
3. A non-2xx status returns `ArchiveCapstoneError::InvalidStatus(status)`.
4. The server responds with a JSON **object** of the form `{ "fetched_at": "<ISO timestamp>", "records": [...] }`. Decode this object as a whole (not as a bare array). A decode failure returns `ArchiveCapstoneError::Decode`.
5. Return `Ok(ArchiveBatch { fetched_at, records })` using the `fetched_at` string provided by the server. Do **not** generate a timestamp from the client clock.

### `persist_archive_batch`
1. Wrap all inserts in a single transaction for atomicity.
2. For each record in `batch.records`:
   - Attempt to insert into the archive table.
   - If the record already exists (duplicate `mission_code` + `artifact`), count it as `skipped` rather than failing.

> **Schema precondition:** `persist_archive_batch` requires the archive table to have a `UNIQUE(mission_code, artifact)` constraint so that `INSERT OR IGNORE` can detect duplicates. Ensure this constraint is present when you create the table (see Quest 29).
   - If successfully inserted, count it as `inserted`.
3. Commit the transaction.
4. Return `Ok(PersistReport { inserted, skipped })`.
5. Wrap SQL errors in `ArchiveCapstoneError::Sql`.

### `load_archive_preview`
1. Select up to `limit` records from the archive table, ordered deterministically (by `mission_code`, then `artifact`).
2. Map each row to `RemoteArchiveRecord`.
3. Return `Ok(Vec<RemoteArchiveRecord>)`.

## Why Staged Design?

`rusqlite::Connection` is not `Send`—it cannot be sent across thread boundaries. Since `async` runtimes may move futures between threads, holding a `&Connection` across an `.await` point violates Rust's safety guarantees (the compiler will reject it).

The staged design solves this: `fetch_archive_batch` is `async` and does all the I/O before touching the database. `persist_archive_batch` is a plain synchronous function that receives the already-fetched data. The caller runs Stage 1 with `.await`, then calls Stage 2 synchronously—`&Connection` never crosses an `await` boundary.

## Concepts Practiced

- **Async/sync boundary:** Why `&Connection` cannot be held across `.await`.
- **Staged pipeline:** Separating async data acquisition from synchronous persistence.
- **`INSERT OR IGNORE`:** Idempotent insert that skips duplicates silently.
- **Transaction wrapping:** All inserts in one atomic batch.
- **`reqwest` + rusqlite together:** Combining both crates in a single function set.

## Setup Notes

Tests use a local **wiremock** mock server for HTTP and `Connection::open_in_memory()` for SQLite. No internet connection or system SQLite installation is required. The first build of World 6 exercises may take 30–45 seconds due to the bundled SQLite compilation. `fetch_archive_batch` tests use `#[tokio::test(flavor = "multi_thread")]`; `persist_archive_batch` and `load_archive_preview` tests are synchronous.

## Edge Cases

- Empty records list from the server (returns `PersistReport { inserted: 0, skipped: 0 }`).
- All records already in the database (all skipped, none inserted).
- Mixed batch: some new, some duplicate.
- `load_archive_preview` with `limit = 0` (returns `Ok(vec![])`).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex34-archive-capstone

# Get a hint if stuck
learn hint ex34-archive-capstone

# Jump to a specific hint level
learn hint ex34-archive-capstone --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**560 XP** for first completion.

## Prerequisites

Complete **Data Migration** (ex33).

## Success Criteria

- `fetch_archive_batch` makes an async GET, decodes the JSON object `{ "fetched_at": "...", "records": [...] }`, and returns the server-provided `fetched_at` string unchanged.
- `persist_archive_batch` is a **synchronous** function (no `async`).
- Duplicate records are silently skipped and counted.
- `PersistReport.inserted + PersistReport.skipped` equals `batch.records.len()`.
- `load_archive_preview` returns at most `limit` records in deterministic order.
- `&Connection` does not appear in any `async fn`.

## What's Next?

**Congratulations!** You've completed the entire learn-rust campaign. You now command:

- **Foundations:** Functions, variables, control flow, strings.
- **Ownership:** Moves, borrows, mutable references.
- **Collections & Errors:** Vectors, maps, Option, Result, custom errors.
- **Deep Signal:** Sync and async HTTP, JSON serde, Tokio, concurrent futures.
- **Parallel Ops:** Threads, channels, Arc/Mutex/RwLock, rayon data parallelism.
- **Archive Core:** SQLite CRUD, transactions, schema migrations, and the async/sync bridge.

**You are a Sovereign. ⬢**

Next steps:
- Build something real with what you've learned.
- Explore [tokio.rs](https://tokio.rs) for production async patterns.
- Read [Rust Async Book](https://rust-lang.github.io/async-book/) for deeper async internals.
- Try [Actix Web](https://actix.rs) or [Axum](https://github.com/tokio-rs/axum) for building HTTP services.
- Explore [SeaORM](https://www.sea-ql.org/SeaORM/) or [Diesel](https://diesel.rs) for production database patterns.

---

**Total campaign XP:** 10,730 | **Final rank:** Sovereign ⬢
