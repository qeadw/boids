# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A boids flocking simulation built as a single HTML file using React 18 (via CDN), Babel for JSX transpilation, and Tailwind CSS for styling. The simulation features bird-like entities ("boids") with emergent flocking behavior, predator-prey dynamics, a day/night cycle, seasonal changes, and a genetic mutation system.

## Running the Project

Open `boids.html` directly in a browser. No build step or server required.

## Security

All secrets and API keys must be stored in `.env` (git-ignored). Never commit credentials, tokens, or keys to the repository.

## Architecture

The entire application lives in `boids.html` with inline `<script type="text/babel">` containing:

### Core Classes (ES6 classes, not React components)
- **Vector2**: 2D vector math (add, sub, mult, normalize, limit, distance)
- **SpatialHash**: Grid-based spatial partitioning for O(1) neighbor queries (cell size 50px)
- **Boid**: Main entity with flocking behavior, traits (bravery, hunger, laziness, sociability), genetic mutations, and lifecycle
- **Predator**: Hunts boids using spatial hash, has pack behavior and leader designation
- **Bug**: Small prey entities that spawn from trees

### Environment Classes
- **Tree** / **Perch**: Rest spots for fatigued boids
- **Shelter**: Protection zones from predators
- **Pond**: Fishing location with drowning risk
- **FoodSource** / **Nest** / **DangerZone** / **Corpse**

### React Component
- **App**: Single component managing all state via `useRef` (game state) and `useState` (UI state)
- Animation loop in `useEffect` with `requestAnimationFrame`
- Settings stored in `settingsRef` (popCap, gameSpeed, mutationMult, birthMult, etc.)

### Key Patterns
- Game state in `stateRef.current` to avoid re-renders during animation
- Spatial hash rebuilt every tick for efficient neighbor queries
- `Boid.flock()` implements separation, alignment, cohesion with predator avoidance
- Mutations are inherited through `parentMutations` object with Mendelian-style probability
- Day/night phase affects speed, visibility, and predator aggression

### Mutation System
27 mutation types defined in `mutationInfo` object with categories: good, bad, neutral, special. Mutations affect `sizeMultiplier`, `speedMultiplier`, `fatigueResistance`, `maxEnergy`, and behavioral flags.
