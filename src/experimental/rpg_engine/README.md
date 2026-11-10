3. rpg_engine

Goal: Establish a reusable role-playing simulation domain.

Focus on:

Core entity and character representations.

Attributes, statistics, and derived values.

Items, inventories, and equipment.

Actions and explicit state transitions.

Deterministic rules and unit tests.

Milestone: A small, testable simulation can create a character, represent its state, and apply a valid action with predictable results.

Start with the domain model—not a complete game, renderer, or campaign system.

// section

set_name	Change the player's name
set_stat	Change a specified stat after validation
gain_experience	Apply experience and any level-up rules
equip_item	Validate and equip an item
unequip_item	Remove an equipped item
add_item	Add an item to inventory
remove_item	Remove an item under defined rules

4. The first playable development milestone
I'd work in this order:
1. Inspect and consolidate the existing player module structure.
2. Establish the public player API and preserve the working display.
3. Introduce the first validated modification functions.
4. Build an interactive player CLI that can inspect and modify the same player instance.
5. Add tests proving that changes persist, invalid inputs are rejected, and unrelated stats or equipment remain unchanged.

One important thing to test next: does the same player instance survive across commands? Your transcript proves that the interfaces work, but not yet that a modified player will retain its changes after returning to a menu or reopening the RPG interface.
That's our next architectural checkpoint.
Mission recommendation
I'd tackle the player system in this order:
1. Player ownership — establish where the live player instance resides.
2. Modification functions — implement validated operations on that instance.
3. Player CLI — expose those operations through interactive commands.
4. Tests — verify modifications, invalid input, and state persistence.
5. Next game domain — move into items and inventory once the player foundation is dependable.
No need to rush into world generation or quests while our player is still becoming a proper game entity.

levels
exp
stats