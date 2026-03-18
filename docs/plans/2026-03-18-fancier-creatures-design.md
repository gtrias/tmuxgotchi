# Fancier Creatures Design

**Date:** 2026-03-18  
**Status:** Approved

## Overview

Redesign all 12 tmuxgotchi creatures to be larger, more detailed, and more expressive. Focus on visual polish through gradient shading and creature-specific idle personalities.

## Specifications

### Size

| Aspect | Current | New |
|--------|---------|-----|
| Height | 5 lines | 7 lines |
| Width | 10 chars | 12 chars |
| Idle frames | 2 | 3-4 |
| Other state frames | 2 | 2 |

### Anatomy Template (7 lines)

```
Line 1: Accessories (ears, antenna, hat, flower)
Line 2: Top of head with shading
Line 3: Eyes + expression
Line 4: Mouth + face detail
Line 5: Body / arms
Line 6: Lower body / legs
Line 7: Feet / ground / effects
```

### Visual Style

- **Gradient shading** using `░▒▓█` for depth and 3D feel
- All sprites padded to exactly 12 chars wide for alignment
- Consistent style across all 12 creatures

**Example — Blob with gradient shading:**
```
            
   ▄▓██▓▄   
  █▒░  ░▒█  
 █░ ◕  ◕ ░█ 
 █░  ◡   ░█ 
  █▒░░░░▒█  
   ▀▀██▀▀   
```

## Creature Idle Personalities

Each creature gets a unique idle quirk animation:

| Creature | Idle Quirk | Frames | Description |
|----------|-----------|--------|-------------|
| **Blob** | Jiggles & settles | 3 | Body wobbles side-to-side, then relaxes |
| **Snail** | Retreats into shell | 4 | Head slowly tucks in, eyes peek out, extends again |
| **Cat** | Grooms & yawns | 4 | Licks paw, yawns with eyes squeezed shut |
| **Robot** | Runs diagnostics | 3 | Antenna spins, screen shows `...` → `OK` |
| **Ghost** | Phases through floor | 4 | Sinks down, becomes translucent `░`, rises back |
| **Bird** | Preens feathers | 3 | Turns head, pecks at wing, fluffs up |
| **Octopus** | Tentacle wave | 4 | Tentacles ripple left-to-right like a wave |
| **Cactus** | Flower blooms | 3 | Bud appears, opens to flower `✿`, petals drift |
| **Mushroom** | Releases spores | 3 | Spores `°` float up in different patterns |
| **Alien** | Shifty eyes | 3 | Eyes move left, right, then blink sideways |
| **Frog** | Catches fly | 4 | Fly buzzes near `·`, tongue zaps out `═`, gulp |
| **Pumpkin** | Candle flickers | 3 | Inner glow alternates `░▒▓`, eyes shift |

**Sleep indicator:** All idle animations include `z` → `zZ` → `zZz` floating nearby.

## Other States

### Working State
- Universal sparkle `✨` animation (position alternates)
- Eyes alert and focused `◕ ◕`
- Creature-specific "active pose" (Cat's tail up, Robot's antenna lit, etc.)
- 2 frames, fast cycle
- Color: Green

### Input State
- Pulsing `!` or `❗` alternates position
- Alert expression `ò ó` (raised eyebrows)
- Creature-specific alert pose:
  - Cat: ears fully perked
  - Robot: antenna extended + `!` on screen
  - Ghost: fully opaque (not fading)
  - Frog: eyes wide, tongue ready
- 2 frames
- Color: Yellow/Orange

### New State (Egg)
- Universal egg design — same for all creatures
- Subtle gradient shading on egg
- Trembles/rocks side-to-side
- Small cracks appear in later frames
- 2-3 frames
- Color: Blue

```
   ╭───╮    
  │░▒░▒░│   
  │▒░◦░▒│   
  │░▒░▒░│   
   ╰───╯    
     ◡      
```

## Implementation

### Files to Modify

| File | Changes |
|------|---------|
| `src/sprites/frames.rs` | Replace all sprite constants with new 7×12 designs, add extra frames for idle animations |
| `src/sprites/mod.rs` | No changes needed (API stays the same) |
| `src/ui/tamagotchi.rs` | Minor layout tweaks if needed |

### Frame Structure

```rust
// New: 3-4 frames for idle, 7 lines each
const BLOB_IDLE: &[&[&str]] = &[
    &[/* frame 1 - 7 lines */],
    &[/* frame 2 - 7 lines */],
    &[/* frame 3 - 7 lines */],
];
```

### Animation Timing

- Keep existing: `frame_idx = ((tick / 4) as usize) % frames.len()`
- More frames = longer cycle automatically

### Width Consistency

- All sprites padded to exactly 12 chars wide
- Trailing spaces preserved for alignment

## Scope

All 12 creatures redesigned:
1. Blob
2. Snail
3. Cat
4. Robot
5. Ghost
6. Bird
7. Octopus
8. Cactus
9. Mushroom
10. Alien
11. Frog
12. Pumpkin

Each creature × 4 states = 48 sprite sets total.
