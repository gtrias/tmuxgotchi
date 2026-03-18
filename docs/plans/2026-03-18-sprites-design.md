# Sprites Tamagotchi Millorats — Design Document

**Date:** 2026-03-18
**Status:** Approved

## Overview

Millorar els sprites de tmuxgotchi amb 12 criatures úniques en estil halfblock pixel-art, cadascuna amb animacions específiques per criatura i per estat. L'objectiu és diversió i whimsy — crear personalitat i attachment emocional amb cada sessió pi.

## Design Decisions

- **Varietat per sessió**: Cada sessió té una criatura única (hash del session_id)
- **12 criatures**: Màxima varietat
- **Estil halfblocks**: Pixel-art clàssic amb `▀▄█░▓`, cohesiu i visible a distància
- **Animacions elaborades**: Cada criatura té animació pròpia segons el seu caràcter

## Criatures

| # | Criatura | Forma distintiva | Animació pròpia |
|---|----------|------------------|-----------------|
| 1 | Blob | Rodó, amorf | Wobble (es deforma) |
| 2 | Cargol | Closca espiral | Antenes es mouen |
| 3 | Gat | Orelles + cua | Cua oscil·la |
| 4 | Robot | Quadrat + antena | Antena fa llums |
| 5 | Fantasma | Flotant, base ondulada | Flota amunt/avall |
| 6 | Ocell | Bec + ales | Ales baten |
| 7 | Pop | Tentacles | Tentacles ondulen |
| 8 | Cactus | Braços + espines | Flor obre/tanca |
| 9 | Bolet | Barret rodó | Espores surten |
| 10 | Alien | Ulls grossos + antenes | Antenes giren |
| 11 | Granota | Ulls saltats | Llengua entra/surt |
| 12 | Carbassa | Cara tallada | Llum interior parpelleja |

## Estats i expressions

| Estat | Expressió facial | Element comú | Color |
|-------|------------------|--------------|-------|
| Working | Ulls contents (◡◡, ^^) | Sparkles ✨ | Verd (`Color::Green`) |
| Idle | Ulls tancats (──, zz) | Zzz flotant | Gris (`Color::DarkGray`) |
| Input | Ulls enfadats (ಠಠ, >.<) | ! parpellejant | Taronja (`Color::Yellow` / `Rgb(255,165,0)`) |
| New | N/A (és un ou) | Tremolor ocasional | Blau (`Color::Blue`) |

## Especificacions tècniques

### Mida dels sprites

- **Alçada**: 5-6 línies (consistent per totes les criatures)
- **Amplada**: 9-11 caràcters (permet variació per forma)
- **Frames d'animació**: 2-4 per combinació criatura/estat

### Assignació de criatura

```rust
fn creature_for_session(session_id: &str) -> CreatureType {
    // Simple hash per assignar criatura determinísticament
    let hash: usize = session_id.bytes().map(|b| b as usize).sum();
    CreatureType::ALL[hash % 12]
}
```

### Estructura de dades

```rust
pub enum CreatureType {
    Blob, Snail, Cat, Robot, Ghost, Bird, 
    Octopus, Cactus, Mushroom, Alien, Frog, Pumpkin,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub name: &'static str,
}

impl Creature {
    /// Get animation frames for this creature in the given state
    pub fn frames(&self, status: SessionStatus, tick: u64) -> Vec<Span>;
}
```

### Organització de fitxers

```
src/
├── sprites/
│   ├── mod.rs           # CreatureType enum, Creature struct, creature_for_session()
│   ├── frames.rs        # Frame data per totes les criatures/estats
│   ├── blob.rs          # Frames específics del blob
│   ├── snail.rs         # Frames específics del cargol
│   ├── cat.rs           # ...
│   ├── robot.rs
│   ├── ghost.rs
│   ├── bird.rs
│   ├── octopus.rs
│   ├── cactus.rs
│   ├── mushroom.rs
│   ├── alien.rs
│   ├── frog.rs
│   └── pumpkin.rs
└── ui/
    └── tamagotchi.rs    # Usa sprites::Creature per renderitzar
```

## Exemples de sprites

### Blob

```
WORKING frame 1:     WORKING frame 2:
   ▄███▄               ▄███▄  
  █ ◕ ◕ █ ✨          █ ◕ ◕ █  ✨
  █  ◡  █             █  ◡  █ 
   █████               █████  
    ▀▀▀                 ▀▀▀   

IDLE frame 1:        IDLE frame 2:
   ▄███▄               ▄███▄  
  █ – – █             █ – – █  z
  █  ω  █             █  ω  █ zZ
   █████               █████  
    ▀▀▀                 ▀▀▀   

INPUT frame 1:       INPUT frame 2:
   ▄███▄  !            ▄███▄  
  █ ò ó █             █ ò ó █ !
  █  △  █             █  △  █ 
   █████               █████  
    ▀▀▀                 ▀▀▀   

NEW frame 1:         NEW frame 2:
    ╭─╮                 ╭─╮   
   │◦ ◦│               │ ◦◦│  
   │ ◦ │               │◦  │  
    ╰─╯                 ╰─╯   
```

### Gat

```
WORKING frame 1:     WORKING frame 2:
   ▄ ▄                 ▄ ▄    
  █◕ ◕█ ✨            █◕ ◕█  ✨
  █ ◡ █               █ ◡ █  
   ███                 ███    
  █ █ ╲               █ █ ╱   
  (cua dreta)         (cua esquerra)

IDLE frame 1:        IDLE frame 2:
   ▄ ▄                 ▄ ▄    
  █– –█               █– –█  z
  █ ω █               █ ω █ zZ
   ███                 ███    
  █ █                 █ █     
```

### Fantasma

```
WORKING frame 1:     WORKING frame 2:
   ▄███▄               ▄███▄  
  █ ◕ ◕ █ ✨            █ ◕ ◕ █✨
  █  ○  █             █  ○  █ 
  ▐█████▌             ▐█████▌ 
   ╲╱╲╱╲               ╱╲╱╲╱  
  (flota baix)        (flota alt)
```

### Robot

```
WORKING frame 1:     WORKING frame 2:
    ▄█▄                 ▄█▄   
   █▀█▀█               █▀█▀█  
  ▐█◕ ◕█▌✨           ▐█◕ ◕█▌ ✨
  ▐█ ▽ █▌             ▐█ ▽ █▌ 
   █████               █████  
  ▐▌   ▐▌             ▐▌   ▐▌ 
  (antena ●)          (antena ○)
```

## Integració amb tamagotchi.rs

```rust
// A render_creature():
let creature = sprites::creature_for_session(&session.session_id);
let frame_idx = (tick / 4) % creature.frame_count(session.status);
let lines = creature.render(session.status, frame_idx);

for (i, line) in lines.iter().enumerate() {
    let span = Span::styled(line, Style::default().fg(status_color));
    // render span at position
}
```

## Testing

- Verificar que les 12 criatures renderitzen correctament
- Verificar que cada estat mostra l'expressió correcta
- Verificar que les animacions ciclen suaument
- Verificar que creature_for_session() distribueix uniformement

## YAGNI

- ❌ Criatures desbloquejables
- ❌ Personalització de colors per criatura
- ❌ Sons/notificacions
- ❌ Guardar criatura preferida per sessió
