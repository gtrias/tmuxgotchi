# Floating Tokens Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add floating energy tokens to creature rooms that visualize remaining context capacity.

**Architecture:** Tokens are rendered as a background layer before creatures. Token positions drift each tick. Token count and type depend on context_pct from PiSession.

**Tech Stack:** Rust, ratatui, emoji rendering

---

## Task 1: Add Token State Management

**Files:**
- Modify: `src/app.rs`

**Step 1: Add token position struct**

Add to `src/app.rs`:

```rust
use rand::Rng;

#[derive(Clone)]
pub struct FloatingToken {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl FloatingToken {
    pub fn new_random(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(2.0..(width as f32 - 2.0)),
            y: rng.gen_range(1.0..(height as f32 - 3.0)),
            vx: rng.gen_range(-0.3..0.3),
            vy: rng.gen_range(-0.2..0.2),
        }
    }

    pub fn update(&mut self, width: u16, height: u16) {
        self.x += self.vx;
        self.y += self.vy;

        // Bounce off walls
        if self.x < 2.0 || self.x > (width as f32 - 4.0) {
            self.vx = -self.vx;
            self.x = self.x.clamp(2.0, width as f32 - 4.0);
        }
        if self.y < 1.0 || self.y > (height as f32 - 4.0) {
            self.vy = -self.vy;
            self.y = self.y.clamp(1.0, height as f32 - 4.0);
        }
    }
}
```

**Step 2: Add tokens to App struct**

```rust
pub struct App {
    // ... existing fields ...
    pub room_tokens: HashMap<String, Vec<FloatingToken>>,
}
```

Initialize in `App::new()`:
```rust
room_tokens: HashMap::new(),
```

**Step 3: Add dependency**

Run: `cargo add rand`

**Step 4: Verify compilation**

Run: `cargo build`

**Step 5: Commit**

```bash
git add src/app.rs Cargo.toml Cargo.lock
git commit -m "feat(tokens): add floating token state management"
```

---

## Task 2: Add Token Update Logic

**Files:**
- Modify: `src/app.rs`

**Step 1: Add token update method to App**

```rust
impl App {
    pub fn update_tokens(&mut self, room_name: &str, context_pct: Option<f32>, width: u16, height: u16) {
        let remaining_pct = 100.0 - context_pct.unwrap_or(0.0);
        let target_count = ((remaining_pct / 100.0) * 12.0).round() as usize;

        let tokens = self.room_tokens.entry(room_name.to_string()).or_insert_with(Vec::new);

        // Adjust token count
        while tokens.len() < target_count {
            tokens.push(FloatingToken::new_random(width, height));
        }
        while tokens.len() > target_count {
            tokens.pop();
        }

        // Update positions
        for token in tokens.iter_mut() {
            token.update(width, height);
        }
    }

    pub fn get_token_char(context_pct: Option<f32>) -> &'static str {
        let remaining_pct = 100.0 - context_pct.unwrap_or(0.0);
        if remaining_pct >= 70.0 {
            "🔮"
        } else if remaining_pct >= 30.0 {
            "💎"
        } else {
            "⚡"
        }
    }
}
```

**Step 2: Verify compilation**

Run: `cargo build`

**Step 3: Commit**

```bash
git add src/app.rs
git commit -m "feat(tokens): add token count and position update logic"
```

---

## Task 3: Render Tokens in Room View

**Files:**
- Modify: `src/ui/tamagotchi.rs`

**Step 1: Add token rendering function**

```rust
fn render_tokens(
    frame: &mut Frame,
    app: &mut App,
    room_name: &str,
    sessions: &[&PiSession],
    area: Rect,
) {
    // Get average context_pct for the room
    let avg_context_pct = if sessions.is_empty() {
        None
    } else {
        let sum: f32 = sessions.iter()
            .filter_map(|s| s.context_pct)
            .sum();
        let count = sessions.iter().filter(|s| s.context_pct.is_some()).count();
        if count > 0 {
            Some(sum / count as f32)
        } else {
            None
        }
    };

    // Update token positions
    app.update_tokens(room_name, avg_context_pct, area.width, area.height);

    // Get token character based on context level
    let token_char = App::get_token_char(avg_context_pct);

    // Render each token
    if let Some(tokens) = app.room_tokens.get(room_name) {
        for token in tokens {
            let x = area.x + token.x.round() as u16;
            let y = area.y + token.y.round() as u16;
            
            if x < area.x + area.width - 1 && y < area.y + area.height - 1 {
                let span = Span::raw(token_char);
                let buf = frame.buffer_mut();
                buf.set_string(x, y, token_char, Style::default());
            }
        }
    }
}
```

**Step 2: Call render_tokens in render_room**

In `render_room`, after rendering the block but before rendering creatures:

```rust
fn render_room(
    frame: &mut Frame,
    app: &mut App,  // Change from &App to &mut App
    room_name: &str,
    sessions: &[&PiSession],
    room_num: usize,
    tick: u64,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} {} ", room_num, room_name));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if sessions.is_empty() {
        return;
    }

    // Render floating tokens first (background layer)
    render_tokens(frame, app, room_name, sessions, inner);

    // Then render creatures on top
    // ... rest of existing code ...
}
```

**Step 3: Update function signatures up the chain**

Update `render_rooms` to pass `&mut App` instead of `&App`, and update the `render` function signature as well.

**Step 4: Verify compilation**

Run: `cargo build`

**Step 5: Commit**

```bash
git add src/ui/tamagotchi.rs
git commit -m "feat(tokens): render floating tokens in room view"
```

---

## Task 4: Update Render Call Chain for Mutability

**Files:**
- Modify: `src/ui/tamagotchi.rs`
- Modify: `src/app.rs` (if needed)

**Step 1: Update render function signature**

Change the main render function to take `&mut App`:

```rust
pub fn render(frame: &mut Frame, app: &mut App) {
    // ...
}
```

**Step 2: Update render_rooms signature**

```rust
fn render_rooms(frame: &mut Frame, app: &mut App, area: Rect) {
    // ...
}
```

**Step 3: Update main.rs if needed**

In the main render loop, ensure app is passed as mutable.

**Step 4: Verify compilation**

Run: `cargo build`

**Step 5: Commit**

```bash
git add src/ui/tamagotchi.rs src/main.rs
git commit -m "refactor(ui): update render chain for mutable app access"
```

---

## Task 5: Add Center Exclusion Zone

**Files:**
- Modify: `src/app.rs`

**Step 1: Add exclusion zone check**

Update `FloatingToken::update` to avoid center area:

```rust
impl FloatingToken {
    pub fn update(&mut self, width: u16, height: u16) {
        self.x += self.vx;
        self.y += self.vy;

        // Bounce off walls
        if self.x < 2.0 || self.x > (width as f32 - 4.0) {
            self.vx = -self.vx;
            self.x = self.x.clamp(2.0, width as f32 - 4.0);
        }
        if self.y < 1.0 || self.y > (height as f32 - 4.0) {
            self.vy = -self.vy;
            self.y = self.y.clamp(1.0, height as f32 - 4.0);
        }

        // Avoid center zone (where creature renders)
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let exclusion_w = 8.0;
        let exclusion_h = 5.0;

        if (self.x - center_x).abs() < exclusion_w && (self.y - center_y).abs() < exclusion_h {
            // Push token away from center
            if self.x < center_x {
                self.x = center_x - exclusion_w;
            } else {
                self.x = center_x + exclusion_w;
            }
        }
    }

    pub fn new_random(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut token = Self {
            x: rng.gen_range(2.0..(width as f32 - 2.0)),
            y: rng.gen_range(1.0..(height as f32 - 3.0)),
            vx: rng.gen_range(-0.3..0.3),
            vy: rng.gen_range(-0.2..0.2),
        };
        // Initial update to push out of exclusion zone
        token.update(width, height);
        token
    }
}
```

**Step 2: Verify compilation**

Run: `cargo build`

**Step 3: Commit**

```bash
git add src/app.rs
git commit -m "feat(tokens): add center exclusion zone to avoid creature overlap"
```

---

## Task 6: Handle Emoji Width

**Files:**
- Modify: `src/ui/tamagotchi.rs`

**Step 1: Account for emoji double-width**

Emoji typically render as 2 characters wide. Update position calculation:

```rust
// In render_tokens, when calculating x position:
let x = area.x + (token.x.round() as u16).min(area.width.saturating_sub(3));
```

**Step 2: Verify and commit**

```bash
cargo build
git add src/ui/tamagotchi.rs
git commit -m "fix(tokens): account for emoji double-width rendering"
```

---

## Task 7: Final Testing and Polish

**Step 1: Build release**

```bash
cargo build --release
```

**Step 2: Visual test**

Run: `cargo run -- view`

Test:
- [ ] Tokens appear in rooms with sessions
- [ ] Token count reflects context usage
- [ ] Tokens change type (🔮→💎→⚡) as context fills
- [ ] Tokens float/drift smoothly
- [ ] Tokens avoid creature center area
- [ ] Multiple rooms have independent token states

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat(tokens): complete floating token implementation

- Tokens visualize remaining context capacity
- 🔮 at 70-100%, 💎 at 30-70%, ⚡ at 0-30%
- Smooth floating animation
- Center exclusion zone around creatures"
```
