# Floating Tokens Design

**Date:** 2026-03-18  
**Status:** Approved

## Overview

Add floating energy tokens to creature rooms that visualize remaining context capacity. Tokens appear as emoji that drift around the room, creating an ambient "energy field" effect.

## Token System

| Context Remaining | Token | Count | Color Meaning |
|-------------------|-------|-------|---------------|
| 70-100% | 🔮 | 8-12 | Calm, plenty of runway |
| 30-70% | 💎 | 5-8 | Moderate, still good |
| 0-30% | ⚡ | 2-4 | Low energy, running hot |

## Visual Behavior

### Token Count Formula
```
token_count = (context_remaining_pct / 100.0 * 12.0).round() as usize
```

- Maximum: 12 tokens at 100% capacity
- Minimum: 0 tokens at 0% capacity
- Token type switches at 70% and 30% thresholds

### Animation

- Tokens float/drift slowly around the room
- Each tick, token positions shift by 1-2 characters in random direction
- Positions wrap around room boundaries
- Tokens avoid overlapping creature sprite area (center exclusion zone)

### Rendering Order

1. Room border and title
2. **Tokens (background layer)**
3. Creature sprite (foreground)
4. Status info below creature

## Visual Examples

**Fresh session (95% capacity):**
```
┌─ ~/src/myproject ─────────────────┐
│     🔮        🔮           🔮     │
│  🔮      ▄▓██▓▄      🔮          │
│        █▒░  ░▒█          🔮      │
│   🔮  █░ ◕  ◕ ░█    🔮           │
│       █░  ◡   ░█                 │
│  🔮    █▒░░░░▒█       🔮    🔮   │
│          ▀▀██▀▀                  │
│        Blob Working              │
└───────────────────────────────────┘
```

**Mid session (50% capacity):**
```
┌─ ~/src/myproject ─────────────────┐
│            💎                     │
│  💎     ▄▓██▓▄          💎       │
│        █▒░  ░▒█                  │
│       █░ ◕  ◕ ░█                 │
│       █░  ◡   ░█     💎          │
│  💎    █▒░░░░▒█                  │
│          ▀▀██▀▀                  │
│        Blob Working              │
└───────────────────────────────────┘
```

**Nearly full (15% capacity):**
```
┌─ ~/src/myproject ─────────────────┐
│                                   │
│         ▄▓██▓▄        ⚡          │
│        █▒░  ░▒█                  │
│       █░ –  – ░█                 │
│       █░  ω   ░█                 │
│   ⚡   █▒░░░░▒█                  │
│          ▀▀██▀▀                  │
│         Blob Idle                │
└───────────────────────────────────┘
```

## Implementation Details

### Data Structure

```rust
struct FloatingToken {
    x: f32,           // Float for smooth movement
    y: f32,
    velocity_x: f32,  // Drift direction
    velocity_y: f32,
}

struct RoomTokens {
    tokens: Vec<FloatingToken>,
    last_update: Instant,
}
```

### Token Position Constraints

- Must stay within room boundaries (1 char padding from edges)
- Exclusion zone: center area where creature sprite renders
- Positions stored as f32, rendered as u16 (rounded)

### Context Capacity Source

From `PiSession`:
- `context_used: Option<u64>` — tokens used
- `context_max: Option<u64>` — context window size
- Calculate: `remaining_pct = 1.0 - (used / max)`

### Files to Modify

| File | Changes |
|------|---------|
| `src/ui/tamagotchi.rs` | Add token rendering layer, token state management |
| `src/app.rs` | Store token positions per room (or regenerate each frame) |
| `src/session.rs` | Ensure context values are parsed (may already exist) |

### Emoji Terminal Considerations

- Emoji are typically rendered as 2-character width
- May need to account for this in position calculations
- Fallback to UTF-8 symbols (`✦`, `◆`, `★`) if emoji cause issues
