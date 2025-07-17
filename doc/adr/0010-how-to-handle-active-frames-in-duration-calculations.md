# 10. How to handle active frames in duration calculations

Date: 2025-07-17

## Status

Accepted

## Context

Watson frames can be active (no stop time) or completed (with stop time). We need to decide how to calculate duration for active frames in total duration calculations.

## Decision

We will **calculate active frame duration from start to current time** instead of excluding them:

- `Frame::duration()` always returns valid `chrono::Duration`
- Active frames: calculate `chrono::Utc::now() - start`
- Completed frames: calculate `stop - start`

## Consequences

- All frames contribute to total duration calculations
- Real-time tracking of active work sessions
- Simplified API without Option handling
