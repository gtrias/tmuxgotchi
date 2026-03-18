# tmuxgotchi

> Inspired by [gavraz/recon](https://github.com/gavraz/recon), a tmux-native dashboard for Claude Code agents.

A tamagotchi-style TUI for managing [pi](https://github.com/mariozechner/pi-coding-agent) agent sessions in tmux.

Run multiple pi sessions in tmux, then manage them all without leaving the terminal — see what each agent is working on, which ones need attention, switch between them, kill or spawn new ones. All from a single keybinding.

![tmuxgotchi tamagotchi view](assets/tamagotchi-view.png)

## Views

### Tamagotchi View (`tmuxgotchi view` or press `v`)

A visual dashboard where each agent is a pixel-art creature living in a room. Designed for a side monitor — glance over and instantly see who's working, sleeping, or idle.

**12 unique creatures** — each session gets a creature based on `hash(session_id) % 12`:

| Creature | Description |
|----------|-------------|
| 🟢 Blob | Friendly goo ball |
| 🐌 Snail | Slow but steady |
| 🐱 Cat | Tail wags when working |
| 🤖 Robot | Antenna blinks |
| 👻 Ghost | Floats up/down |
| 🐦 Bird | Wings flap |
| 🐙 Octopus | Tentacles wave |
| 🌵 Cactus | Flower blooms |
| 🍄 Mushroom | Spore particles |
| 👽 Alien | Eyes shift |
| 🐸 Frog | Tongue catches bugs |
| 🎃 Pumpkin | Jack-o-lantern glow |

**States** — each creature has unique animations per state:

| State | Animation | Color |
|-------|-----------|-------|
| **Working** | Sparkles ✨ | Green |
| **Idle** | Sleeping zzZ | Grey |
| **New** | Egg trembles | Blue |
| **Input** | Alert ! (pulsing) | Orange |

- **Rooms** group agents by working directory (2×2 grid, paginated)
- **Zoom** into a room with `1`-`4`, page with `j`/`k`

### Table View (default)

```
┌─ tmuxgotchi ─────────────────────────────────────────────────────────────────────┐
│  #  Session       Project::Branch          Status    Model       Context  Cost   │
│  1  dev:1         myapp::main              ● Work    Opus 4.6    24%/1M   $1.20  │
│  2  dev:2         backend::feat/api        ● Idle    Opus 4.6    7%/1M    $0.45  │
│  3  work:1        frontend::develop        ● Idle    Sonnet 4.6  10%/200k $0.80  │
└──────────────────────────────────────────────────────────────────────────────────┘
j/k navigate  Enter switch  x kill  v tamagotchi  n new  r refresh  q quit
```

## How it works

tmuxgotchi discovers pi sessions by:

1. **tmux list-panes** — finds panes running `pi` processes
2. **tmux capture-pane** — reads the pi status bar for model, context, cache info
3. **~/.pi/agent/sessions/** — parses JSONL files for session data and costs
4. **Status detection** — spinners (`⠋⠙⠹...`) + "Working..." = agent is busy

```
┌─────────────────────────────────────────────────────────────┐
│                      tmux server                             │
│  ┌───────────────┐  ┌───────────────┐  ┌──────────────┐     │
│  │ session:pane  │  │ session:pane  │  │ session:pane │     │
│  │      pi       │  │      pi       │  │      pi      │     │
│  └───────┬───────┘  └───────┬───────┘  └──────┬───────┘     │
└──────────┼──────────────────┼─────────────────┼─────────────┘
           │                  │                 │
           ▼                  ▼                 ▼
     ┌──────────────────────────────────────────────┐
     │              tmuxgotchi (TUI)                 │
     └──────────────────────────────────────────────┘
```

## Install

```bash
cargo install --path .
```

Requires tmux and [pi](https://github.com/mariozechner/pi-coding-agent).

## Usage

```bash
tmuxgotchi                    # Table dashboard (default)
tmuxgotchi view               # Tamagotchi visual dashboard
tmuxgotchi json               # JSON output (for scripting)
tmuxgotchi launch             # Create a new pi session in current directory
tmuxgotchi launch --cwd ~/src/myproject  # New session in specific dir
tmuxgotchi next               # Jump to next working agent
```

### Keybindings — Table View

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate sessions |
| `Enter` | Switch to selected tmux pane |
| `x` | Kill selected session |
| `v` | Switch to Tamagotchi view |
| `n` | Launch new session |
| `i` / `Tab` | Jump to next working agent |
| `r` | Force refresh |
| `q` / `Esc` | Quit |

### Keybindings — Tamagotchi View

| Key | Action |
|-----|--------|
| `1`-`4` | Zoom into room |
| `j` / `k` | Previous / next page |
| `h` / `l` | Select agent (when zoomed) |
| `Enter` | Switch to selected agent |
| `x` | Kill selected agent |
| `n` | New session in room |
| `Esc` | Zoom out (or quit) |
| `v` | Switch to table view |
| `r` | Force refresh |

## tmux config

Add to your `~/.tmux.conf`:

```bash
bind g display-popup -E -w 80% -h 60% "tmuxgotchi"        # prefix + g → dashboard
bind G display-popup -E -w 80% -h 60% "tmuxgotchi view"   # prefix + G → tamagotchi
bind N display-popup -E -w 80% -h 60% "tmuxgotchi launch" # prefix + N → new session  
bind I run-shell "tmuxgotchi next"                         # prefix + I → jump to working
```

## License

MIT
