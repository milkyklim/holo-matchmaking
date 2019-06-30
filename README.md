## Nim implementation on Holochain

### Holochain DevCamp - June 2019

Reference: [wikipedia.org](https://en.wikipedia.org/wiki/Nim).

Implementation: [https://github.com/milkyklim/holo-nim-game](holo-nim-game).

**Rules:**

- Game starts with 3 piles of `[x_, y, z]` stones.
- Players take turns: one player can take more than 1 turn in a row.
- Player has to remove at least one stone from the pile.
- Player can't remove stones from more than one pile simultaneously.
- Player who can't remove a stone loses.

**Move description:**

```
{
    game: "QmHashOfGame123",
    author: "QmMyAgentAddress000",
    previous_move: "QmHashOfPreviousMove"
    move_type: {
        RemoveStone: {
            from: {x: 1, y: 2, z: 3},
            to: {x: 1, y: 2, z: 0}.
        }
    }
}
```

**Game state:**

```
{
    complete: false,
    stones: [{x: 1, y: 2, z: 3}],
    player_1: {
        resigned: false,
    }
    player_2: {
        resigned: false,
    }
}
```
