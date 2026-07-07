# Kingdom Two Crowns Clone — Development Plan (Milestones)

> Living development plan. Each milestone is a vertical slice of functionality.
> Some milestones may be split into multiple tutorial episodes once implementation reveals the scope.
> Ordering is based on natural development flow, not a 1-to-1 Bevy-API-to-episode mapping.

---

## Milestone 1: The Moving Monarch

**Goal:** A window with a player character that moves left and right, plays a sprite animation, and has a camera following them over a simple background.

**Why first:** A controllable player and visible motion are the minimum needed to test anything else. A static sprite is also not fun, so animation belongs here.

**Bevy patterns used:** `App`, `DefaultPlugins`, `Plugin`, `Camera2d`, `OrthographicProjection`, `Input`, `Transform`, `Time`, `TextureAtlas`, sprite animation, `Sprite.flip_x`.

**Result at the end of this milestone:** A monarch-on-horse sprite that animates when moving, flips to face direction, rides left/right, and is followed by a camera over a scrolling ground and sky.

---

## Milestone 2: A Generated Island

**Goal:** Generate a simple side-scrolling island from a seed, so the player can walk around and see different terrain.

**Why second:** Movement needs a world to move through. Once the player can ride, the next natural step is to give them an island to ride across.

**Bevy patterns used:** `Resource` (`WorldSeed`), `rand` + `SeedableRng`, `Commands`, `Query`, `Transform`, `Sprite`, asset loading.

**Result at the end of this milestone:** Trees, ground variation, a campfire, and a starting castle spawned from a seed. The player can ride around and inspect the world.

---

## Milestone 3: Day and Night

**Goal:** A global day/night cycle that tints the world and drives a “day” / “night” state.

**Why here:** This is primarily for atmosphere and pacing. It also introduces a state machine before combat, which is useful. It could be implemented later if needed.

**Bevy patterns used:** `Resource` (`DayCycle`), `Timer`, `Time`, `State` / `ComputedStates`, `NextState`, `ClearColor`, color interpolation.

**Result at the end of this milestone:** The screen darkens and lightens over time; the game knows whether it is day or night.

---

## Milestone 4: Coins and the Purse

**Goal:** The monarch can drop coins on the ground and pick them back up; a HUD shows the coin count.

**Why here:** The economy is the foundation of recruitment and building. Physical coins also introduce events and spawning.

**Bevy patterns used:** `Event` (`DropCoin`, `PickupCoin`), `EventReader` / `EventWriter`, `Resource` (`CoinPurse`), `Query`, `despawn`, UI (`Node`, `Text`).

**Result at the end of this milestone:** Drop a coin with one key, walk over it to pick it up, see the count on screen.

---

## Milestone 5: The Campfire and Wanderers

**Goal:** Mirror the Kingdom Two Crowns recruitment flow: wanderers appear at the campfire over time; the monarch drops a coin near a wanderer; the wanderer picks it up and transforms into an idle villager.

**Why here:** This is the first true Kingdom Two Crowns mechanic. It connects economy, NPCs, and the campfire scene.

**Bevy patterns used:** `bsn!`, `Observer` / `on()`, `SceneComponent`, `Timer`, spawning, hierarchy, zone/radius detection, component swapping.

**Result at the end of this milestone:** A campfire scene where wanderers spawn periodically; drop a coin near one and it becomes an idle villager.

**Scope note:** This milestone may be too large for one tutorial episode. It can be split into “Wanderers arrive at the campfire” and “Recruiting a villager” once the scope is clearer.

---

## Milestones 6+

Intentionally left blank. The next batch will be planned after the first five milestones are implemented and their actual scope is known.

Likely upcoming areas (not yet ordered):

- Villager jobs (builder, archer, farmer, knight)
- Trees, harvesting, and forest regrowth
- Walls, farms, towers, and upgrade tiers
- The Greed: AI, night waves, escalation, blood moon
- Archers, arrows, and melee combat
- Crown loss and game over
- Save / load
- Boat travel and new islands
- Sound, music, particles, screenshake
- Menus, pause, and game over screen
- Optional local/online co-op
