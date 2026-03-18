# Fancier Creatures Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign all 12 creatures to be 7×12 with gradient shading and unique idle personalities.

**Architecture:** Replace all sprite constants in `frames.rs` with new designs. Each creature gets 4 states (Working, Idle, Input, New). Idle state gets 3-4 frames with personality quirks; other states get 2 frames.

**Tech Stack:** Rust, Unicode box-drawing characters (`░▒▓█▄▀▐▌`)

---

## Task 0: Create Universal Egg (New State)

**Files:**
- Modify: `src/sprites/frames.rs` — replace `BLOB_NEW` constant

**Step 1: Replace the egg sprite**

Replace the current `BLOB_NEW` constant with the new 7-line gradient egg:

```rust
const EGG_NEW: &[&[&str]] = &[
    &[
        "            ",
        "   ╭────╮   ",
        "  │░▒░▒░▒│  ",
        "  │▒░ ◦ ░▒│ ",
        "  │░▒░▒░▒│  ",
        "   ╰────╯   ",
        "            ",
    ],
    &[
        "            ",
        "   ╭────╮   ",
        "  │▒░▒░▒░│  ",
        "  │░▒ ◦ ▒░│ ",
        "  │▒░▒░▒░│  ",
        "   ╰────╯   ",
        "     ◡      ",
    ],
    &[
        "            ",
        "  ╭────╮    ",
        "  │░▒░▒░▒│  ",
        "  │▒░ ◦ ░▒│ ",
        "  │░▒░▒░▒│  ",
        "   ╰────╯   ",
        "            ",
    ],
];
```

**Step 2: Update all creature NEW references**

Change all `BLOB_NEW` references to `EGG_NEW`:
- `SNAIL_NEW`, `CAT_NEW`, `ROBOT_NEW`, `GHOST_NEW`, `BIRD_NEW`
- `OCTOPUS_NEW`, `CACTUS_NEW`, `MUSHROOM_NEW`, `ALIEN_NEW`, `FROG_NEW`, `PUMPKIN_NEW`

**Step 3: Verify compilation**

Run: `cargo build`
Expected: Compiles without errors

**Step 4: Visual test**

Run: `cargo run -- view`
Expected: Any "New" state agents show the new gradient egg

**Step 5: Commit**

```bash
git add src/sprites/frames.rs
git commit -m "feat(sprites): add universal gradient egg for New state"
```

---

## Task 1: Redesign Blob

**Files:**
- Modify: `src/sprites/frames.rs` — replace `BLOB_WORKING`, `BLOB_IDLE`, `BLOB_INPUT`

**Step 1: Replace Blob Working sprite**

```rust
const BLOB_WORKING: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ ◕  ◕ ░█✨",
        " █░  ◡   ░█ ",
        "  █▒░░░░▒█  ",
        "   ▀▀██▀▀   ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄ ✨",
        "  █▒░  ░▒█  ",
        " █░ ◕  ◕ ░█ ",
        " █░  ◡   ░█ ",
        "  █▒░░░░▒█  ",
        "   ▀▀██▀▀   ",
    ],
];
```

**Step 2: Replace Blob Idle sprite (jiggle animation)**

```rust
const BLOB_IDLE: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        " █░  ω   ░█ ",
        "  █▒░░░░▒█  ",
        "   ▀▀██▀▀ z ",
    ],
    &[
        "            ",
        "  ▄▓██▓▄    ",
        " █▒░  ░▒█   ",
        "█░ –  – ░█  ",
        "█░  ω   ░█  ",
        " █▒░░░░▒█   ",
        "  ▀▀██▀▀ zZ ",
    ],
    &[
        "            ",
        "    ▄▓██▓▄  ",
        "   █▒░  ░▒█ ",
        "  █░ –  – ░█",
        "  █░  ω   ░█",
        "   █▒░░░░▒█ ",
        "    ▀▀██▀▀zZ",
    ],
];
```

**Step 3: Replace Blob Input sprite**

```rust
const BLOB_INPUT: &[&[&str]] = &[
    &[
        "          ! ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ ò  ó ░█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "   ▀▀██▀▀   ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄ ! ",
        "  █▒░  ░▒█  ",
        " █░ ò  ó ░█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "   ▀▀██▀▀   ",
    ],
];
```

**Step 4: Verify compilation**

Run: `cargo build`

**Step 5: Visual test**

Run: `cargo run -- view`
Start a test pi session and observe Blob in different states

**Step 6: Commit**

```bash
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Blob with gradient shading and jiggle idle"
```

---

## Task 2: Redesign Snail

**Files:**
- Modify: `src/sprites/frames.rs` — replace `SNAIL_WORKING`, `SNAIL_IDLE`, `SNAIL_INPUT`

**Step 1: Replace Snail Working sprite**

```rust
const SNAIL_WORKING: &[&[&str]] = &[
    &[
        "    ¤   ¤   ",
        "    █▓▓▓█   ",
        "   █░◕◕░█ ✨",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀  ",
        "   ══════   ",
    ],
    &[
        "    ¤   ¤   ",
        "    █▓▓▓█ ✨",
        "   █░◕◕░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀  ",
        "   ══════   ",
    ],
];
```

**Step 2: Replace Snail Idle sprite (shell retreat)**

```rust
const SNAIL_IDLE: &[&[&str]] = &[
    &[
        "    ¤   ¤   ",
        "    █▓▓▓█   ",
        "   █░––░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀ z",
        "   ══════   ",
    ],
    &[
        "            ",
        "    █▓▓▓█   ",
        "   █░◦◦░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀zZ",
        "   ══════   ",
    ],
    &[
        "            ",
        "            ",
        "   █░··░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀zZ",
        "   ══════   ",
    ],
    &[
        "            ",
        "    █▓▓▓█   ",
        "   █░◦◦░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀zZ",
        "   ══════   ",
    ],
];
```

**Step 3: Replace Snail Input sprite**

```rust
const SNAIL_INPUT: &[&[&str]] = &[
    &[
        "    ¤   ¤ ! ",
        "    █▓▓▓█   ",
        "   █░òó░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀  ",
        "   ══════   ",
    ],
    &[
        "    ¤   ¤   ",
        "    █▓▓▓█ ! ",
        "   █░òó░█   ",
        "  ▄█▒▒▒▒█▄  ",
        " █▓██████▓█ ",
        "  ▀▀▀▀▀▀▀▀  ",
        "   ══════   ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Snail with shell retreat idle"
```

---

## Task 3: Redesign Cat

**Files:**
- Modify: `src/sprites/frames.rs` — replace `CAT_WORKING`, `CAT_IDLE`, `CAT_INPUT`

**Step 1: Replace Cat Working sprite**

```rust
const CAT_WORKING: &[&[&str]] = &[
    &[
        "  ▲     ▲   ",
        "  █▓███▓█   ",
        " █░ ◕ ◕ ░█✨",
        " █░  ω  ░█  ",
        "  █▒███▒█   ",
        "  █░█ █░█ ╱ ",
        "   ▀   ▀    ",
    ],
    &[
        "  ▲     ▲ ✨",
        "  █▓███▓█   ",
        " █░ ◕ ◕ ░█  ",
        " █░  ω  ░█  ",
        "  █▒███▒█   ",
        "  █░█ █░█ ╲ ",
        "   ▀   ▀    ",
    ],
];
```

**Step 2: Replace Cat Idle sprite (groom & yawn)**

```rust
const CAT_IDLE: &[&[&str]] = &[
    &[
        "  ▲     ▲   ",
        "  █▓███▓█   ",
        " █░ – – ░█  ",
        " █░  ω  ░█  ",
        "  █▒███▒█  z",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
    &[
        "  ▲     ▲   ",
        "  █▓███▓█   ",
        " █░ – – ░█  ",
        " █░  ω  ░█ɱ ",
        "  █▒█░░▒█ zZ",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
    &[
        "  ▲     ▲   ",
        "  █▓███▓█   ",
        " █░ ˃ ˂ ░█  ",
        " █░  O  ░█  ",
        "  █▒███▒█zZz",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
    &[
        "  ▲     ▲   ",
        "  █▓███▓█   ",
        " █░ – – ░█  ",
        " █░  ω  ░█  ",
        "  █▒███▒█ zZ",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
];
```

**Step 3: Replace Cat Input sprite**

```rust
const CAT_INPUT: &[&[&str]] = &[
    &[
        "  ▲     ▲ ! ",
        "  █▓███▓█   ",
        " █░ ò ó ░█  ",
        " █░  ^  ░█  ",
        "  █▒███▒█   ",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
    &[
        "  ▲▲   ▲▲   ",
        "  █▓███▓█ ! ",
        " █░ ò ó ░█  ",
        " █░  ^  ░█  ",
        "  █▒███▒█   ",
        "  █░█ █░█   ",
        "   ▀   ▀    ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Cat with groom/yawn idle"
```

---

## Task 4: Redesign Robot

**Files:**
- Modify: `src/sprites/frames.rs` — replace `ROBOT_WORKING`, `ROBOT_IDLE`, `ROBOT_INPUT`

**Step 1: Replace Robot Working sprite**

```rust
const ROBOT_WORKING: &[&[&str]] = &[
    &[
        "     ●      ",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ ◕ ◕ ░▌✨",
        " ▐░ ▀▀▀ ░▌  ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
    &[
        "     ○    ✨",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ ◕ ◕ ░▌  ",
        " ▐░ ▀▀▀ ░▌  ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
];
```

**Step 2: Replace Robot Idle sprite (diagnostics)**

```rust
const ROBOT_IDLE: &[&[&str]] = &[
    &[
        "     ◐      ",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ – – ░▌  ",
        " ▐░ ... ░▌ z",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
    &[
        "     ◓      ",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ – – ░▌  ",
        " ▐░ ..  ░▌zZ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
    &[
        "     ◑      ",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ – – ░▌  ",
        " ▐░ OK  ░▌zZ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
];
```

**Step 3: Replace Robot Input sprite**

```rust
const ROBOT_INPUT: &[&[&str]] = &[
    &[
        "     █    ! ",
        "    ▄█▄     ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ ò ó ░▌  ",
        " ▐░  !  ░▌  ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
    &[
        "     ██     ",
        "    ▄█▄   ! ",
        "  ▐█▓▓▓█▌   ",
        " ▐░ ò ó ░▌  ",
        " ▐░  !  ░▌  ",
        "  ▐█▓▓▓█▌   ",
        "   █▀ ▀█    ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Robot with diagnostics idle"
```

---

## Task 5: Redesign Ghost

**Files:**
- Modify: `src/sprites/frames.rs` — replace `GHOST_WORKING`, `GHOST_IDLE`, `GHOST_INPUT`

**Step 1: Replace Ghost Working sprite**

```rust
const GHOST_WORKING: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█✨",
        " █░ ◕  ◕ ░█ ",
        " █░  ○   ░█ ",
        "  ▐▓████▓▌  ",
        "   ╲╱╲╱╲╱   ",
    ],
    &[
        "          ✨",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ ◕  ◕ ░█ ",
        " █░  ○   ░█ ",
        "  ▐▓████▓▌  ",
        "   ╱╲╱╲╱╲   ",
    ],
];
```

**Step 2: Replace Ghost Idle sprite (phase through floor)**

```rust
const GHOST_IDLE: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        " █░  ○   ░█ ",
        "  ▐▓████▓▌ z",
        "   ╲╱╲╱╲╱   ",
    ],
    &[
        "            ",
        "            ",
        "   ▄░██░▄   ",
        "  █░ –– ░█  ",
        " █░  ○   ░█ ",
        "  ▐░████░▌zZ",
        "   ╲╱╲╱╲╱   ",
    ],
    &[
        "            ",
        "            ",
        "            ",
        "   ░░░░░░   ",
        "  ░ ○○ ░░  ",
        "   ░░░░░░ zZ",
        "   ~~~~~~   ",
    ],
    &[
        "            ",
        "            ",
        "   ▄░██░▄   ",
        "  █░ –– ░█  ",
        " █░  ○   ░█ ",
        "  ▐░████░▌zZ",
        "   ╲╱╲╱╲╱   ",
    ],
];
```

**Step 3: Replace Ghost Input sprite**

```rust
const GHOST_INPUT: &[&[&str]] = &[
    &[
        "          ! ",
        "   ▄▓██▓▄   ",
        "  █▓░  ░▓█  ",
        " █▓ ò  ó ▓█ ",
        " █▓  ○   ▓█ ",
        "  ▐▓████▓▌  ",
        "   ╲╱╲╱╲╱   ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄ ! ",
        "  █▓░  ░▓█  ",
        " █▓ ò  ó ▓█ ",
        " █▓  ○   ▓█ ",
        "  ▐▓████▓▌  ",
        "   ╲╱╲╱╲╱   ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Ghost with floor-phase idle"
```

---

## Task 6: Redesign Bird

**Files:**
- Modify: `src/sprites/frames.rs` — replace `BIRD_WORKING`, `BIRD_IDLE`, `BIRD_INPUT`

**Step 1: Replace Bird Working sprite**

```rust
const BIRD_WORKING: &[&[&str]] = &[
    &[
        "    ▄▓▓▄    ",
        "   █░░░░█   ",
        "  █ ◕  ◕ █✨",
        " ◄█  ▼▼  █  ",
        "   █▓▓▓▓█   ",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
    &[
        "    ▄▓▓▄  ✨",
        "   █░░░░█   ",
        "  █ ◕  ◕ █  ",
        "  █  ▼▼  █► ",
        "   █▓▓▓▓█   ",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
];
```

**Step 2: Replace Bird Idle sprite (preen feathers)**

```rust
const BIRD_IDLE: &[&[&str]] = &[
    &[
        "    ▄▓▓▄    ",
        "   █░░░░█   ",
        "  █ –  – █  ",
        "  █  ▼▼  █  ",
        "   █▓▓▓▓█  z",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
    &[
        "    ▄▓▓▄    ",
        "   █░░░░█   ",
        "  █ –    █  ",
        "  █  ▼▼° █◄ ",
        "   █▓▓▓▓█ zZ",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
    &[
        "    ▄▓▓▄    ",
        "   █░░░░█ ~ ",
        "  █ –  – █  ",
        "  █  ▼▼  █  ",
        "   █▓▓▓▓█zZz",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
];
```

**Step 3: Replace Bird Input sprite**

```rust
const BIRD_INPUT: &[&[&str]] = &[
    &[
        "    ▄▓▓▄  ! ",
        "   █░░░░█   ",
        "  █ ò  ó █  ",
        "  █  ▼▼  █  ",
        "   █▓▓▓▓█   ",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
    &[
        "    ▄▓▓▄    ",
        "   █░░░░█ ! ",
        "  █ ò  ó █  ",
        "  █  ▼▼  █  ",
        "   █▓▓▓▓█   ",
        "    █▒▒█    ",
        "    ▌▐▌▐    ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Bird with preen idle"
```

---

## Task 7: Redesign Octopus

**Files:**
- Modify: `src/sprites/frames.rs` — replace `OCTOPUS_WORKING`, `OCTOPUS_IDLE`, `OCTOPUS_INPUT`

**Step 1: Replace Octopus Working sprite**

```rust
const OCTOPUS_WORKING: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█✨",
        " █░ ◕  ◕ ░█ ",
        "  █▓████▓█  ",
        " ╱│╲ ▓▓ ╱│╲ ",
        "     ▓▓     ",
    ],
    &[
        "          ✨",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ ◕  ◕ ░█ ",
        "  █▓████▓█  ",
        " ╲│╱ ▓▓ ╲│╱ ",
        "     ▓▓     ",
    ],
];
```

**Step 2: Replace Octopus Idle sprite (tentacle wave)**

```rust
const OCTOPUS_IDLE: &[&[&str]] = &[
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        "  █▓████▓█ z",
        " ╱╲  ▓▓  ╱╲ ",
        "     ▓▓     ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        "  █▓████▓█zZ",
        " ╲╱╲ ▓▓ ╱╲╱ ",
        "     ▓▓     ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        "  █▓████▓█zZ",
        "  ╲╱ ▓▓ ╲╱  ",
        "     ▓▓     ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ –  – ░█ ",
        "  █▓████▓█zZ",
        " ╱╲╱ ▓▓ ╱╲╱ ",
        "     ▓▓     ",
    ],
];
```

**Step 3: Replace Octopus Input sprite**

```rust
const OCTOPUS_INPUT: &[&[&str]] = &[
    &[
        "          ! ",
        "   ▄▓██▓▄   ",
        "  █▒░  ░▒█  ",
        " █░ ò  ó ░█ ",
        "  █▓████▓█  ",
        " ╱│╲ ▓▓ ╱│╲ ",
        "     ▓▓     ",
    ],
    &[
        "            ",
        "   ▄▓██▓▄ ! ",
        "  █▒░  ░▒█  ",
        " █░ ò  ó ░█ ",
        "  █▓████▓█  ",
        " ╲│╱ ▓▓ ╲│╱ ",
        "     ▓▓     ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Octopus with tentacle wave idle"
```

---

## Task 8: Redesign Cactus

**Files:**
- Modify: `src/sprites/frames.rs` — replace `CACTUS_WORKING`, `CACTUS_IDLE`, `CACTUS_INPUT`

**Step 1: Replace Cactus Working sprite**

```rust
const CACTUS_WORKING: &[&[&str]] = &[
    &[
        "     ▄█▄ ✿  ",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ ◕◕ ░▓█✨",
        " █░  ◡   ░█ ",
        "  █▒░░░░▒█  ",
        "    ▀██▀    ",
    ],
    &[
        "     ▄█▄✿ ✨",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ ◕◕ ░▓█ ",
        " █░  ◡   ░█ ",
        "  █▒░░░░▒█  ",
        "    ▀██▀    ",
    ],
];
```

**Step 2: Replace Cactus Idle sprite (flower blooms)**

```rust
const CACTUS_IDLE: &[&[&str]] = &[
    &[
        "     ▄█▄ •  ",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ –– ░▓█ ",
        " █░  ω   ░█ ",
        "  █▒░░░░▒█ z",
        "    ▀██▀    ",
    ],
    &[
        "     ▄█▄ ❀  ",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ –– ░▓█ ",
        " █░  ω   ░█ ",
        "  █▒░░░░▒█zZ",
        "    ▀██▀    ",
    ],
    &[
        "     ▄█▄ ✿ °",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ –– ░▓█ ",
        " █░  ω   ░█°",
        "  █▒░░░░▒█zZ",
        "    ▀██▀    ",
    ],
];
```

**Step 3: Replace Cactus Input sprite**

```rust
const CACTUS_INPUT: &[&[&str]] = &[
    &[
        "     ▄█▄  ! ",
        "    ▄███▄   ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ òó ░▓█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "    ▀██▀    ",
    ],
    &[
        "     ▄█▄    ",
        "    ▄███▄ ! ",
        "  ▄█▒░░░▒█▄ ",
        " █▓░ òó ░▓█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "    ▀██▀    ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Cactus with flower bloom idle"
```

---

## Task 9: Redesign Mushroom

**Files:**
- Modify: `src/sprites/frames.rs` — replace `MUSHROOM_WORKING`, `MUSHROOM_IDLE`, `MUSHROOM_INPUT`

**Step 1: Replace Mushroom Working sprite**

```rust
const MUSHROOM_WORKING: &[&[&str]] = &[
    &[
        "  ▄▓████▓▄  ",
        " █▒ ●  ● ▒█ ",
        " █░ ●  ● ░█✨",
        "  ▀▓████▓▀  ",
        "    █▒▒█    ",
        "    █░░█   °",
        "    ▀▀▀▀  ° ",
    ],
    &[
        "  ▄▓████▓▄✨",
        " █▒ ●  ● ▒█ ",
        " █░ ●  ● ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█   °",
        "    █░░█  ° ",
        "    ▀▀▀▀    ",
    ],
];
```

**Step 2: Replace Mushroom Idle sprite (spore release)**

```rust
const MUSHROOM_IDLE: &[&[&str]] = &[
    &[
        "  ▄▓████▓▄  ",
        " █▒ ●  ● ▒█ ",
        " █░ –  – ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█   z",
        "    █░░█    ",
        "    ▀▀▀▀    ",
    ],
    &[
        "  ▄▓████▓▄ °",
        " █▒ ●  ● ▒█ ",
        " █░ –  – ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█  zZ",
        "    █░░█    ",
        "    ▀▀▀▀    ",
    ],
    &[
        "  ▄▓████▓▄° ",
        " █▒ ●  ● ▒█°",
        " █░ –  – ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█ zZz",
        "    █░░█    ",
        "    ▀▀▀▀    ",
    ],
];
```

**Step 3: Replace Mushroom Input sprite**

```rust
const MUSHROOM_INPUT: &[&[&str]] = &[
    &[
        "  ▄▓████▓▄! ",
        " █▒ ●  ● ▒█ ",
        " █░ ò  ó ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█    ",
        "    █░░█    ",
        "    ▀▀▀▀    ",
    ],
    &[
        "  ▄▓████▓▄  ",
        " █▒ ●  ● ▒█!",
        " █░ ò  ó ░█ ",
        "  ▀▓████▓▀  ",
        "    █▒▒█    ",
        "    █░░█    ",
        "    ▀▀▀▀    ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Mushroom with spore release idle"
```

---

## Task 10: Redesign Alien

**Files:**
- Modify: `src/sprites/frames.rs` — replace `ALIEN_WORKING`, `ALIEN_IDLE`, `ALIEN_INPUT`

**Step 1: Replace Alien Working sprite**

```rust
const ALIEN_WORKING: &[&[&str]] = &[
    &[
        "   ◢▓▓▓▓◣   ",
        "  █▒░░░░▒█  ",
        " █░ ◉  ◉ ░█✨",
        " █░  ▽   ░█ ",
        "  █▒░░░░▒█  ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
    &[
        "   ◢▓▓▓▓◣ ✨",
        "  █▒░░░░▒█  ",
        " █░  ◉◉  ░█ ",
        " █░  ▽   ░█ ",
        "  █▒░░░░▒█  ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
];
```

**Step 2: Replace Alien Idle sprite (shifty eyes)**

```rust
const ALIEN_IDLE: &[&[&str]] = &[
    &[
        "   ◢▓▓▓▓◣   ",
        "  █▒░░░░▒█  ",
        " █░◉    ░░█ ",
        " █░  ▽   ░█ ",
        "  █▒░░░░▒█ z",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
    &[
        "   ◢▓▓▓▓◣   ",
        "  █▒░░░░▒█  ",
        " █░░    ◉░█ ",
        " █░  ▽   ░█ ",
        "  █▒░░░░▒█zZ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
    &[
        "   ◢▓▓▓▓◣   ",
        "  █▒░░░░▒█  ",
        " █░ ━━━  ░█ ",
        " █░  ▽   ░█ ",
        "  █▒░░░░▒█zZ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
];
```

**Step 3: Replace Alien Input sprite**

```rust
const ALIEN_INPUT: &[&[&str]] = &[
    &[
        "   ◢▓▓▓▓◣ ! ",
        "  █▒░░░░▒█  ",
        " █░ ◉  ◉ ░█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
    &[
        "   ◢▓▓▓▓◣   ",
        "  █▒░░░░▒█! ",
        " █░ ◉  ◉ ░█ ",
        " █░  △   ░█ ",
        "  █▒░░░░▒█  ",
        "   █▓▓▓▓█   ",
        "  ▀▀    ▀▀  ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Alien with shifty eyes idle"
```

---

## Task 11: Redesign Frog

**Files:**
- Modify: `src/sprites/frames.rs` — replace `FROG_WORKING`, `FROG_IDLE`, `FROG_INPUT`

**Step 1: Replace Frog Working sprite**

```rust
const FROG_WORKING: &[&[&str]] = &[
    &[
        "   ◉   ◉    ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█✨",
        " █░  ▽  ░█  ",
        "  ▀▓███▓▀   ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
    &[
        "   ◉   ◉  ✨",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█  ",
        " █░ ▽══ ░█  ",
        "  ▀▓███▓▀   ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
];
```

**Step 2: Replace Frog Idle sprite (catches fly)**

```rust
const FROG_IDLE: &[&[&str]] = &[
    &[
        "   –   –  · ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█  ",
        " █░  ω  ░█  ",
        "  ▀▓███▓▀  z",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
    &[
        "   ◉   ◉ ·  ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█  ",
        " █░  ▽  ░█  ",
        "  ▀▓███▓▀ zZ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
    &[
        "   ◉   ◉    ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█══",
        " █░  ▽  ░█  ",
        "  ▀▓███▓▀ zZ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
    &[
        "   ◉   ◉    ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█  ",
        " █░  ω  ░█  ",
        "  ▀▓███▓▀zZz",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
];
```

**Step 3: Replace Frog Input sprite**

```rust
const FROG_INPUT: &[&[&str]] = &[
    &[
        "   ◉   ◉  ! ",
        "  ▄▓███▓▄   ",
        " █▒░░░░░▒█  ",
        " █░  △  ░█  ",
        "  ▀▓███▓▀   ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
    &[
        "   ◉   ◉    ",
        "  ▄▓███▓▄ ! ",
        " █▒░░░░░▒█  ",
        " █░  △  ░█  ",
        "  ▀▓███▓▀   ",
        "   █▒ ▒█    ",
        "  █▀   ▀█   ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Frog with fly-catching idle"
```

---

## Task 12: Redesign Pumpkin

**Files:**
- Modify: `src/sprites/frames.rs` — replace `PUMPKIN_WORKING`, `PUMPKIN_IDLE`, `PUMPKIN_INPUT`

**Step 1: Replace Pumpkin Working sprite**

```rust
const PUMPKIN_WORKING: &[&[&str]] = &[
    &[
        "     ▄▄     ",
        "  ▄▓████▓▄  ",
        " █▒░░░░░░▒█✨",
        " █░▀◕▽◕▀░█ ",
        " █░ ▀▀▀ ░█  ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
    &[
        "     ▄▄   ✨",
        "  ▄▓████▓▄  ",
        " █▒░░░░░░▒█ ",
        " █░▀◕▽◕▀░█ ",
        " █░ ▀▀▀ ░█  ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
];
```

**Step 2: Replace Pumpkin Idle sprite (candle flickers)**

```rust
const PUMPKIN_IDLE: &[&[&str]] = &[
    &[
        "     ▄▄     ",
        "  ▄▓████▓▄  ",
        " █▒░░░░░░▒█ ",
        " █░▀–▽–▀░█ ",
        " █▒ ▀▀▀ ▒█ z",
        "  ▀▓████▓▀  ",
        "            ",
    ],
    &[
        "     ▄▄     ",
        "  ▄▓████▓▄  ",
        " █▒░░▓░░░▒█ ",
        " █▒▀–▽–▀▒█ ",
        " █░ ▀▀▀ ░█zZ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
    &[
        "     ▄▄     ",
        "  ▄▓████▓▄  ",
        " █▒░▓▓▓░░▒█ ",
        " █▓▀–▽–▀▓█ ",
        " █▒ ▀▀▀ ▒█zZ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
];
```

**Step 3: Replace Pumpkin Input sprite**

```rust
const PUMPKIN_INPUT: &[&[&str]] = &[
    &[
        "     ▄▄   ! ",
        "  ▄▓████▓▄  ",
        " █▒░░░░░░▒█ ",
        " █░▀ò▽ó▀░█ ",
        " █░ ▀▀▀ ░█  ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
    &[
        "     ▄▄     ",
        "  ▄▓████▓▄! ",
        " █▒░░░░░░▒█ ",
        " █░▀ò▽ó▀░█ ",
        " █░ ▀▀▀ ░█  ",
        "  ▀▓████▓▀  ",
        "            ",
    ],
];
```

**Step 4: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "feat(sprites): redesign Pumpkin with candle flicker idle"
```

---

## Task 13: Update Frame References

**Files:**
- Modify: `src/sprites/frames.rs` — update all `*_NEW` references

**Step 1: Replace all NEW state references**

Change:
```rust
const SNAIL_NEW: &[&[&str]] = &[BLOB_NEW[0], BLOB_NEW[1]];
const CAT_NEW: &[&[&str]] = &[BLOB_NEW[0], BLOB_NEW[1]];
// ... etc
```

To:
```rust
const SNAIL_NEW: &[&[&str]] = EGG_NEW;
const CAT_NEW: &[&[&str]] = EGG_NEW;
const ROBOT_NEW: &[&[&str]] = EGG_NEW;
const GHOST_NEW: &[&[&str]] = EGG_NEW;
const BIRD_NEW: &[&[&str]] = EGG_NEW;
const OCTOPUS_NEW: &[&[&str]] = EGG_NEW;
const CACTUS_NEW: &[&[&str]] = EGG_NEW;
const MUSHROOM_NEW: &[&[&str]] = EGG_NEW;
const ALIEN_NEW: &[&[&str]] = EGG_NEW;
const FROG_NEW: &[&[&str]] = EGG_NEW;
const PUMPKIN_NEW: &[&[&str]] = EGG_NEW;
```

Also update `get_frames` match to use `EGG_NEW` for `BLOB_NEW` reference.

**Step 2: Verify and commit**

```bash
cargo build
git add src/sprites/frames.rs
git commit -m "refactor(sprites): unify all NEW states to use EGG_NEW"
```

---

## Task 14: Final Visual Testing

**Step 1: Build release**

```bash
cargo build --release
```

**Step 2: Manual visual test**

Run: `cargo run -- view`

Test each creature type appears correctly:
- [ ] Blob — jiggle animation in idle
- [ ] Snail — shell retreat
- [ ] Cat — groom/yawn
- [ ] Robot — diagnostics
- [ ] Ghost — phase through floor
- [ ] Bird — preen feathers
- [ ] Octopus — tentacle wave
- [ ] Cactus — flower bloom
- [ ] Mushroom — spore release
- [ ] Alien — shifty eyes
- [ ] Frog — catches fly
- [ ] Pumpkin — candle flicker

**Step 3: Verify all states**

- [ ] Working — green, sparkles
- [ ] Idle — grey, zzZ, personality animation
- [ ] Input — yellow, pulsing !
- [ ] New — blue, gradient egg

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat(sprites): complete fancier creatures redesign

- All 12 creatures upgraded to 7×12 size
- Gradient shading with ░▒▓█ characters
- Unique idle personality animations
- Universal gradient egg for New state"
```
