/*!
 * Author: Colton Philips
 *
 * Description:
 *
 * &Str storage of copy text within the game allowing for ease of edit and access.
 * Prefer absense of "magic unnamed variables" in logical code
 *
 * Usage:
 *
 * Notes:
 *
 */

pub const MENU: &str = "
TERMBAN SOKOBAN 1.0 ENGINE
==========================
INTRODUCING... MICROBAN I!
        BY DAVID W. SKINNER

CONTROLS:       Press Enter to Begin
                Press Escape to Leave
                Move Player Using WASD/Arrows
                Press R to Restart Level
                Press Z or U to Undo a Move

RULES:
    1. The PLAYER and BOXES can only occupy EMPTY or GOAL Tiles.
    2. The PLAYER can push a BOX onto an EMPTY Tile.
    3. A BOX cannot be pushed by another BOX.
GOAL:
    The puzzle is solved when every GOAL tile is occupied by a BOX.
";
