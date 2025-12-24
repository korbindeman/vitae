# Chess

A chess game built with Vitae.

## Features

### Implemented
- [x] Board rendering with SVG pieces
- [x] Turn-based play (white/black alternating)
- [x] Piece selection (only own pieces on your turn)
- [x] Legal move validation for all pieces
  - [x] Pawn (forward, two-square start, diagonal capture)
  - [x] Rook (horizontal/vertical, path blocking)
  - [x] Bishop (diagonal, path blocking)
  - [x] Queen (combined rook/bishop)
  - [x] Knight (L-shape, can jump)
  - [x] King (one square any direction)
- [x] Prevent capturing own pieces
- [x] Valid move indicators (squares)
- [x] Board flip option (current player at bottom)
- [x] Last move display
- [x] En passant
- [x] Castling
- [x] Captured pieces display
- [x] Check detection (can't move into check, can't ignore check)

### Not Yet Implemented
- [ ] Pawn promotion
- [ ] Checkmate/stalemate detection
- [ ] Move history list
- [ ] Game reset button
- [ ] Drag and drop movement
- [ ] Sound effects
- [ ] Undo move
- [ ] Timer/clock

## Validation Strategy

### Perft Testing
Move generation correctness is verified via perft (performance test), which counts all possible positions at a given depth. Expected node counts from the starting position:

| Depth | Nodes |
|-------|-------|
| 1 | 20 |
| 2 | 400 |
| 3 | 8,902 |
| 4 | 197,281 |
| 5 | 4,865,609 |

### FEN Support
FEN string parsing enables loading arbitrary positions for edge case testing (pins, discovered checks, castling rights, en passant).

### Test Positions
Specific positions verify correct handling of:
- Pinned pieces (absolute pins to king)
- Discovered checks
- Castling through/out of/into check
- En passant captures that expose check
- Double check (only king can move)
- Stalemate positions

### Regression Tests
Saved game sequences from previously-fixed bugs ensure regressions are caught.

### Fuzzing
Random legal move sequences verify invariants:
- Exactly one king per side
- Pawns never on ranks 1 or 8 (without promotion)
- Turn alternates correctly
- Captured piece counts match removed pieces

### PGN Replays
Grandmaster games parsed from PGN files verify all moves are accepted as legal.

### Illegal Move Rejection
Explicit tests verify rejection of:
- Moving through pieces (rook, bishop, queen)
- Capturing own pieces
- Ignoring check
- Castling without rights
- Invalid piece movement patterns

### Symmetry Testing
Mirrored positions verify identical move counts for white and black.
