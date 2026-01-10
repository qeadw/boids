# Living World - Boids Ecosystem Simulation

## Project Context for Claude Code

This document contains all context needed to continue development on this project. The main file is `index.html` (or `boids.html`), a standalone React/Canvas simulation.

---

## Tech Stack

- **React 18** (via CDN - unpkg)
- **Babel** (via CDN - for JSX transpilation in browser)
- **Tailwind CSS** (via CDN)
- **HTML5 Canvas** for rendering
- **No build step** - runs directly in browser

---

## Architecture Overview

### Core Classes

| Class | Purpose |
|-------|---------|
| `Vector2` | 2D vector math (add, sub, mult, div, normalize, limit, dist, distSq) |
| `SpatialHash` | O(n) neighbor lookups - divides world into 50px cells |
| `Boid` | Bird entity with flocking, mutations, energy, fatigue, aging |
| `Predator` | Hunts boids, pack behavior, evolves across generations |
| `Bug` | Small food source, spawns from trees |
| `FoodSource` | Stationary food patch with depleting amount |
| `Corpse` | Dead boid, can be eaten, decays over time |
| `Tree` | Environmental feature, spawns bugs, has perch |
| `Perch` | Resting spot on trees, limited capacity |
| `Pond` | Water feature with fish, risky fishing (10% drown chance) |
| `Shelter` | Safe zone from predators, limited capacity |
| `Nest` | Breeding location, increases birth rate nearby |
| `Ripple` | Visual effect for births, deaths, events |
| `DangerZone` | Temporary danger marker where kills happened |

### State Management

All game state stored in `stateRef.current`:
```javascript
{
  boids: [],           // All bird entities
  predators: [],       // All predator entities
  obstacles: [],       // Rocks (Vector2 positions)
  food: [],            // FoodSource instances
  trees: [],           // Tree instances (5 default)
  shelters: [],        // Shelter instances (2 default)
  nests: [],           // Nest instances (2 default)
  dangerZones: [],     // DangerZone instances
  ripples: [],         // Visual effects
  bugs: [],            // Bug instances
  corpses: [],         // Corpse instances
  pond: null,          // Single Pond instance
  time: 0,             // Game tick counter
  dayTime: 0,          // Day/night cycle phase
  seasonTime: 0,       // Season cycle phase
  cursorPos: null,     // Mouse position Vector2
  nextPackId: 0,       // Counter for predator packs
  width: 900,          // Canvas width
  height: 450,         // Canvas height
  spatialHash: SpatialHash  // For optimization
}
```

Settings stored in `settingsRef.current`:
```javascript
{
  popCap: 150,         // Max boid population
  gameSpeed: 1,        // Simulation speed multiplier
  mutationMult: 1,     // Mutation chance multiplier
  birthMult: 1,        // Birth rate multiplier
  bugMult: 1,          // Bug spawn rate multiplier
  predBreeding: true,  // Whether predators can breed
  cursorStrength: 1,   // Vortex attract/repel strength
  startBoids: 60       // Initial boid count on reset
}
```

---

## All 27 Mutations

### Good Mutations
| Key | Emoji | Name | Effect |
|-----|-------|------|--------|
| `giant` | 🦣 | Giant | 1.5x size, 0.85x speed |
| `tiny` | 🐜 | Tiny | 0.6x size, 1.2x speed |
| `speedy` | ⚡ | Speedy | 1.4x speed |
| `glowing` | ✨ | Glowing | Bioluminescent glow effect |
| `tough` | 💪 | Tough | 50% fatigue resistance |
| `longlived` | 🕐 | Long-lived | 1.5x lifespan |
| `fertile` | 🥚 | Fertile | 2x reproduction chance |
| `camouflage` | 👻 | Camouflage | Harder to catch, 50% opacity |
| `plated` | 🛡️ | Plated | Survives 1 attack (armor breaks) |
| `nocturnal` | 🦉 | Nocturnal | Boosted at night |
| `rainbow` | 🌈 | Rainbow | Color shifts over time |
| `zen` | 🧘 | Zen | +40% bravery |
| `magnetic` | 🧲 | Magnetic | +50% sociability |
| `bigstomach` | 🎈 | Big Stomach | 200 max energy (2x), 2x food gains |
| `mechanical` | 🤖 | Mechanical | 1.1x speed, 0.3x fatigue, 0.5x energy drain, blue glow |

### Neutral Mutations
| Key | Emoji | Name | Effect |
|-----|-------|------|--------|
| `aggressive` | 😠 | Aggressive | Hunts and kills cannibals |
| `immortal` | ♾️ | Immortal | No age death, 3x energy drain |
| `smallstomach` | 🫘 | Small Stomach | 50 max energy, 0.66x drain |
| `ravenous` | 🍖 | Ravenous | Eats faster, needs more food |

### Bad Mutations
| Key | Emoji | Name | Effect |
|-----|-------|------|--------|
| `cannibal` | 🦷 | Cannibal | Eats other birds when <20% energy |
| `traitor` | 💀 | Traitor | Becomes predator on death |
| `fat` | 🐷 | Fat | 1.3x size, 0.7x speed |
| `paper` | 📄 | Paper | 2x fatigue, very easy to catch |
| `flightless` | 🐧 | Flightless | 0.5x speed, cannot perch |
| `tasty` | 🍗 | Tasty | Predators prioritize hunting you |
| `bullied` | 😢 | Bullied | -40% sociability, always slightly scared |

### Special Mutations
| Key | Emoji | Name | Effect |
|-----|-------|------|--------|
| `egggiver` | 🥚 | Egg-Giver | On death, spawns 3 babies with random mutations (cannot be inherited) |

---

## Boid Behavior Priority (Highest to Lowest)

1. **Flee from predators** (4x multiplier when scared)
2. **Avoid obstacles** (3x when fleeing, 1x normal)
3. **Seek shelter** (when fear > 0.3)
4. **Separation** (avoid crowding)
5. **Alignment** (match velocity with neighbors)
6. **Cohesion** (move toward group center)
7. **Avoid danger zones**
8. **Seek perch** (when fatigued)
9. **Cursor interaction** (attract/repel)
10. **Seek pond** (only when <15% energy AND not scared)

---

## Key Game Mechanics

### Day/Night Cycle
- `dayPhase = (Math.sin(state.dayTime) + 1) / 2`
- Affects: boid speed, predator aggression, nocturnal bonuses
- Night: predators faster/more aggressive, boids slower

### Seasons
- 4 seasons: Spring, Summer, Fall, Winter
- Affects: bug spawn rates, food spawn rates
- Summer: most bugs (60 max), Winter: fewest (20 max)

### Reproduction
- Requires: energy > 75, age > 400, fatigue < 40
- Needs mate nearby (within 25px) with similar requirements
- Near nest: 4x birth chance
- Cross-species: 30% of normal chance, produces hybrid
- Mutations: 50% inherit from one parent, 100% if both have it

### Predator Evolution
- Breeds after: 3 kills, energy > 130
- Offspring: generation+1 (faster, stronger)
- 5% chance to inherit random mutation from prey

### Fishing
- Triggers when boid energy < 15% of max
- Must reach pond center, wait 60 ticks
- 90% chance: catch fish (+25 energy)
- 10% chance: drown (death)

### Edge Behavior
- Normal: pac-man wrap (exit right, enter left)
- When chased by predator: strong inward force near edges (prevents trapping)

---

## UI Tools

| Tool | Emoji | Action |
|------|-------|--------|
| Food | 🌿 | Place food source |
| Obstacle | 🪨 | Place rock (blocks movement) |
| Predator | 👁 | Spawn predator (joins nearby pack or creates new) |
| Shelter | 🏠 | Place shelter |
| Nest | 🪺 | Place nest (random species) |
| Boid | 🐦 | Spawn 8 boids |
| Select | 👆 | Click boid to open mutation panel |
| Favorite | ⭐ | Highlight/track specific boid |

### Cursor Modes
- **None**: No effect
- **Attract** (🧲): Pulls boids toward cursor
- **Repel** (💨): Pushes boids away from cursor
- Strength adjustable via slider (0.1x - 3x)

---

## Performance Optimizations

1. **Spatial Hashing**: 50px cells, O(n) neighbor lookups
2. **distSq()**: Avoid sqrt when possible
3. **Throttled UI**: Stats update every 500ms
4. **Refs over State**: Settings in refs don't trigger re-renders
5. **Limited trails**: Max 6 positions per boid

---

## Known Behaviors / Edge Cases

- Friendship lines only render if friends within 40% screen width (prevents cross-screen lines)
- Traitor mutation doesn't trigger on drowning death
- Egg-giver mutation cannot be inherited
- Predators avoid obstacles but can still catch boids near them
- Aggressive boids actively hunt cannibals
- Plated armor breaks on first hit, logged as "armor_broke" event

---

## File Structure

```
living-world-boids/
├── index.html      # Main game (standalone, no dependencies)
├── README.md       # Project readme
├── .gitignore      # Git ignore file
└── CLAUDE_CONTEXT.md  # This file
```

---

## Potential Future Features

Ideas mentioned or partially implemented:

- [ ] More environmental hazards (storms, earthquakes)
- [ ] Fireflies at night
- [ ] Aurora effects
- [ ] Territory marking
- [ ] More predator types
- [ ] Sound effects
- [ ] Save/load simulation state
- [ ] Statistics graphs over time
- [ ] Mobile touch controls
- [ ] WebGL rendering for better performance at 1000+ boids

---

## Development Commands

```bash
# Run locally (any static server works)
npx serve .
python -m http.server 8000

# No build step needed - edit index.html directly

# Deploy to GitHub Pages
git add .
git commit -m "description"
git push origin main
# Enable Pages in repo settings → main branch → root
```

---

## Code Style Notes

- Classes use ES6 class syntax
- React functional components with hooks
- All game logic in single file (intentional - keeps it simple)
- Minified/compressed style for class methods
- Tailwind utility classes for UI
- No TypeScript (plain JS for browser compatibility)

---

## Last Session Summary

### Features Added This Session:
1. ✅ Pause button
2. ✅ Favorite bird highlighting (⭐ tool)
3. ✅ Vortex strength slider
4. ✅ Start boids count setting
5. ✅ Bugs collide with rocks
6. ✅ Birds collide with rocks
7. ✅ Predators avoid rocks
8. ✅ Birds prioritize fleeing over food
9. ✅ Birds avoid rocks more when fleeing
10. ✅ FPS counter
11. ✅ Spatial hashing optimization
12. ✅ Fixed input focus stealing
13. ✅ Fixed friendship lines across screen
14. ✅ Fixed canvas resize updating world bounds

### Bugs Fixed:
- Input boxes no longer deselect while typing (using defaultValue + onBlur)
- Friendship lines don't draw across screen when friends wrap
- Canvas resize properly updates simulation bounds
- Rocks now block bugs, birds, and predators

---

## Contact / Attribution

Originally developed through conversation with Claude (Anthropic).
