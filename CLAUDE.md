# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a React + TypeScript + Vite application that displays a circular 24-hour clock visualization showing timezone overlaps between Dallas, Connecticut, and London. The clock helps identify optimal meeting times when all three timezones are in working hours (8:00-18:00 local time).

## Common Commands

```bash
# Start development server (runs on http://localhost:5173)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Lint code
npm run lint
```

## Application Architecture

### Entry Point
- `src/main.tsx` - React app initialization, renders `TzApp` component
- `index.html` - Base HTML template

### Core Component
- `src/TzApp.tsx` - Main component containing all application logic
  - **Single-file component**: All logic, state, and rendering are in this one file
  - No separate components, hooks, or utilities are used

### Key Architectural Patterns

**State Management**:
- Uses `useState` for meetings array, selected time slot, and current time
- Meetings structure: `{ id, dallasHour, title, essential }`

**Timezone Logic**:
- All time calculations are Dallas-centric (outer ring)
- Connecticut is +1 hour from Dallas (middle ring)
- London is +6 hours from Dallas (inner ring)
- Work hours: 8:00-18:00 local time in each timezone

**SVG Rendering**:
- Circular clock divided into 24 hour segments
- Three concentric rings for three timezones (Dallas outer, Connecticut middle, London inner)
- Uses polar coordinate math (`polarToCartesian`) to position segments
- Segments colored based on working hours overlap:
  - Green (#4ade80): all three timezones working
  - Light pink (#fce7f3): Dallas working
  - Light blue (#bfdbfe): Connecticut working
  - Light green (#dcfce7): London working
  - White: no one working

**Real-time Updates**:
- `useEffect` timer updates current time every 60 seconds
- Current time highlighted with pink outline across all rings and a radial line

## Development Notes

- This is a single-page application with no routing
- No external state management libraries (Redux, Zustand, etc.)
- No UI component library - uses custom SVG and minimal Tailwind-like classes
- Meetings are stored in component state only (no persistence)
- The app uses Vite's HMR for fast refresh during development
