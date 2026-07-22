## Hint 3: Algorithm Outline

```
function insert_crew_member(conn, entry):
    Step 1: Execute INSERT INTO crew_manifest (crew_id, role, rank)
            VALUES (?1, ?2, ?3)
            Bind: entry.crew_id, entry.role, entry.rank as i64
            → map SQL error to CrewManifestError::Sql

    Step 2: Return Ok(())

function load_crew_manifest(conn):
    Step 1: Prepare SELECT statement:
            SELECT crew_id, role, rank FROM crew_manifest ORDER BY crew_id

    Step 2: Call stmt.query_map([], |row| { ... })
            In the closure:
                → row.get::<_, String>(0) → crew_id
                → row.get::<_, String>(1) → role
                → row.get::<_, i64>(2)    → rank (cast to u8 when building struct)
                → return Ok(CrewManifestEntry { crew_id, role, rank: rank as u8 })

    Step 3: Collect the iterator into Vec<CrewManifestEntry>
            using .collect::<rusqlite::Result<Vec<_>>>()?

    Step 4: Map the error to CrewManifestError::Sql and return Ok(vec)
```

**Note:** `query_map` returns an iterator of `rusqlite::Result<T>`. You must collect and unwrap errors before returning. The `.collect::<Result<Vec<_>, _>>()` idiom does this in one step.

**Spoiler threshold:** High—complete algorithm with the type cast and collection idiom spelled out.
