# Debug Creature Stability

**Problem:** User reports creatures start correctly assigned but become equal after first refresh/turn.

**Hypothesis:**
1. `session_id` changes between refreshes (JSONL file changes, fallback ID generated)
2. Sessions get reordered/mixed during refresh
3. Hash collision (unlikely - tested and hashes are different)

---

## Task 1: Add debug logging to file

**Files:** 
- Create: `src/debug.rs`
- Modify: `src/main.rs`

**Step 1:** Create debug module

```rust
// src/debug.rs
use std::fs::OpenOptions;
use std::io::Write;

const DEBUG_LOG: &str = "/tmp/tmuxgotchi-debug.log";

pub fn log(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DEBUG_LOG)
    {
        let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%H:%M:%S%.3f"), msg);
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        $crate::debug::log(&format!($($arg)*))
    };
}
```

**Step 2:** Add `chrono` dependency

```bash
cargo add chrono
```

**Step 3:** Add module to main.rs

```rust
mod debug;
```

**Step 4:** Commit

```bash
git add -A && git commit -m "feat(debug): add file-based debug logging"
```

---

## Task 2: Log session discovery

**Files:** `src/session.rs`

**Step 1:** Add logging at end of `discover_sessions`

```rust
// At end of discover_sessions, before return:
for s in &sessions {
    debug_log!(
        "DISCOVER: session_id={} project={} status={:?} jsonl={}",
        s.session_id,
        s.project_name,
        s.status,
        s.jsonl_path.display()
    );
}
```

**Step 2:** Log when session_id is generated vs found

In `discover_sessions`, around line where session_id is set:

```rust
let session_id = jsonl_info.session_id.clone().unwrap_or_else(|| {
    let fallback = format!("tmux-{}-{}", pane.session_name, pane.pane_id.replace(':', "-"));
    debug_log!("FALLBACK session_id: {} (no JSONL session_id found)", fallback);
    fallback
});

if jsonl_info.session_id.is_some() {
    debug_log!("JSONL session_id: {}", session_id);
}
```

**Step 3:** Commit

```bash
git add -A && git commit -m "feat(debug): log session discovery details"
```

---

## Task 3: Log creature assignment in render

**Files:** `src/ui/tamagotchi.rs`

**Step 1:** Add logging in `render_creature`

```rust
fn render_creature(frame: &mut Frame, session: &PiSession, tick: u64, area: Rect) {
    let creature = sprites::creature_for_session(&session.session_id);
    
    debug_log!(
        "RENDER: session_id={} -> creature={} (hash={})",
        session.session_id,
        creature.name(),
        session.session_id.bytes().map(|b| b as usize).sum::<usize>() % 12
    );
    
    // ... rest of function
```

**Step 2:** Commit

```bash
git add -A && git commit -m "feat(debug): log creature assignment in render"
```

---

## Task 4: Log refresh cycle

**Files:** `src/app.rs`

**Step 1:** Add logging in `refresh`

```rust
pub fn refresh(&mut self) {
    debug_log!("=== REFRESH START ===");
    debug_log!("prev_sessions count: {}", self.prev_sessions.len());
    
    let sessions = session::discover_sessions(&self.prev_sessions);
    
    debug_log!("new sessions count: {}", sessions.len());
    for s in &sessions {
        debug_log!("  - {} ({}) -> {}", s.project_name, s.session_id, s.status.label());
    }

    self.prev_sessions = sessions
        .iter()
        .map(|s| (s.session_id.clone(), s.clone()))
        .collect();

    self.sessions = sessions;
    debug_log!("=== REFRESH END ===");
    
    // ... rest
}
```

**Step 2:** Commit

```bash
git add -A && git commit -m "feat(debug): log refresh cycle"
```

---

## Task 5: Build and test

**Step 1:** Build

```bash
cargo build --release
```

**Step 2:** Clear old log and run

```bash
rm -f /tmp/tmuxgotchi-debug.log
./target/release/tmuxgotchi view
# Wait for a few refreshes, then quit
```

**Step 3:** Analyze log

```bash
cat /tmp/tmuxgotchi-debug.log
```

**What to look for:**
- `FALLBACK session_id` entries → session_id instability
- Different `session_id` for same project across refreshes
- `creature=X` changing for same session_id

**Step 4:** Commit with findings

```bash
git add -A && git commit -m "debug: creature stability investigation"
```

---

## Task 6: Cleanup (after debugging)

Once issue is found and fixed, remove debug logging:

```bash
rm src/debug.rs
# Remove debug_log! calls from session.rs, app.rs, tamagotchi.rs
# Remove mod debug; from main.rs
cargo remove chrono  # if not needed elsewhere
git add -A && git commit -m "chore: remove debug logging"
```

---

## Expected Log Output

```
[14:32:01.123] === REFRESH START ===
[14:32:01.124] prev_sessions count: 0
[14:32:01.156] JSONL session_id: d054bb77-8b4d-42d2-bc88-6eafee35856d
[14:32:01.157] JSONL session_id: 1059faf0-9595-461e-bca9-7c2b78b9a75f
[14:32:01.158] DISCOVER: session_id=d054bb77-... project=sunflare status=Working
[14:32:01.158] DISCOVER: session_id=1059faf0-... project=deplayer status=Idle
[14:32:01.159] new sessions count: 2
[14:32:01.159] === REFRESH END ===
[14:32:01.200] RENDER: session_id=d054bb77-... -> creature=Alien (hash=9)
[14:32:01.201] RENDER: session_id=1059faf0-... -> creature=Cat (hash=2)
[14:32:03.123] === REFRESH START ===
[14:32:03.124] prev_sessions count: 2
...
```

If creatures become equal, the log will show either:
- Different `session_id` values between refreshes for same project
- `FALLBACK session_id` being generated (JSONL not found)
- Same hash value for different sessions (collision)
