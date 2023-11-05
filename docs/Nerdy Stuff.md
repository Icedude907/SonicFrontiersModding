
## Wild Rush (Crasher) mechanics:

`distanceRatios[5]` - The distances sonic moves relative to the enemy while zig zagging. There is an implicit 6th movement towards the enemy.
- `0` is sonic's starting pos
- `1` is enemy pos
- `-1` is behind starting pos
- `1.5` is behind enemy (opposite to where you started)

`angles[5]`
`radii[5]`
  Consider a line drawn between Sonic's starting position and the enemy.
  A radius value determines Sonic's displacement from this line (right angled, in game units) at any given zig zag point.
  An angle determines the direction he moves in (0 is up, 90 is to the right of sonic when viewed from behind)

The game determines these 6 positions (5 + final on enemy) to move Sonic in and jumps linearly from position to position.

The direction you dodge in when starting the move does not flip the angles. Its always the same.