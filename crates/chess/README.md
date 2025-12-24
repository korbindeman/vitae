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

### Not Yet Implemented
- [ ] En passant
- [ ] Castling
- [ ] Pawn promotion
- [ ] Check detection
- [ ] Checkmate/stalemate detection
- [ ] Move history list
- [ ] Captured pieces display
- [ ] Game reset button
- [ ] Drag and drop movement
- [ ] Sound effects
- [ ] Undo move
- [ ] Timer/clock
