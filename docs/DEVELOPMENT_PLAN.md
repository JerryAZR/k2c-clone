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

## Milestone 6: Builder Job

**Goal:** Lead an idle villager to a builder camp and assign them the builder job.

**Why next:** First job; builders are the foundation for harvesting and construction.

**Bevy patterns used:** enum `Job` component, job state machine, `bsn!` tool-camp scene, proximity detection, component swapping.

**Result at the end of this milestone:** A villager becomes a builder and can be given construction or harvesting tasks.

---

## Milestone 7: Trees and Harvesting

**Goal:** Builders chop trees for coins; forests regenerate over time.

**Why next:** Builders need a simple job before they can construct complex buildings.

**Bevy patterns used:** `Timer`, resource health, events, spawning coins, regrowth scheduling.

**Result at the end of this milestone:** Builders harvest trees, coins drop, and trees regrow after a delay.

---

## Milestone 8: Walls

**Goal:** Builders construct defensive walls from build sites; upgrade tiers.

**Why next:** First building type; walls are needed before archers can defend and before Greed can attack.

**Bevy patterns used:** `BuildSite`, `ConstructionProgress`, upgrade tiers, `SceneComponent`.

**Result at the end of this milestone:** Place a wall build site; a builder walks over and finishes it.

---

## Milestone 9: Archer Job

**Goal:** Lead a villager to an archer camp to make an archer. Archers hunt critters during the day.

**Why next:** Introduces targeting and shooting logic in a safe daytime context before Greed exist. Hunting logic is complex enough to be its own milestone.

**Bevy patterns used:** projectile, targeting, fire-rate timer, `DelayedCommands`, collision.

**Result at the end of this milestone:** Archers automatically shoot nearby critters during the day.

---

## Milestone 10: Towers

**Goal:** Builders construct archer towers; archers can occupy them for extra range.

**Why next:** Towers do not make sense without archers. Once archers exist, giving them a defensive position makes them much more useful.

**Bevy patterns used:** building upgrade chain, `SceneComponent`, entity occupation/slots.

**Result at the end of this milestone:** Archers can occupy towers for extra range.

---

## Milestone 11: Farmers and Farms

**Goal:** Lead a villager to a farm camp to make a farmer. Farms generate coins during the day; farmers return to safety at night.

**Why here:** Adds economy and reinforces that day/night affects friendly NPC behavior before combat pressure.

**Bevy patterns used:** state-dependent NPC behavior, farm income timer, `DayState`/`NightState` gating.

**Result at the end of this milestone:** Farms produce coins during the day; farmers hide at night.

---

## Milestone 12: The Greed

**Goal:** Night spawns waves of Greed from one side; they move toward the crown and attack walls.

**Why next:** Once walls and archers exist, enemies give them a purpose.

**Bevy patterns used:** `NightState`, spawning, basic AI, wave escalation, random attack direction.

**Result at the end of this milestone:** Greed appear at night and move toward the monarch.

---

## Milestone 13: Defense and Crown Loss

**Goal:** Archers shoot Greed at night; walls take damage; crown loss ends the game.

**Why next:** Reuses the archer shooting logic from milestone 9 against real threats.

**Bevy patterns used:** projectile reuse, health/damage, `Observer`, `GameOver` state.

**Result at the end of this milestone:** Archers defend, walls break, and the crown can be lost.

---

## Milestone 14: Menus, Pause, and UI

**Goal:** Main menu, pause overlay, game over screen, polished HUD.

**Why here:** Proper menu and pause flow makes testing and save/load much easier.

**Bevy patterns used:** `State`, `OnEnter`/`OnExit`, `Button`, `Interaction`, UI layout.

**Result at the end of this milestone:** Complete menu and pause flow.

---

## Milestone 15: Save and Load

**Goal:** Persist the kingdom between play sessions.

**Why here:** Core loop is complete; now we can learn about save compatibility and migration.

**Bevy patterns used:** `serde`, `AppTypeRegistry`, serialization, filtering transient entities.

**Result at the end of this milestone:** Save on exit and load on startup.

---

## Milestone 16: Boat and Islands

**Goal:** Builders construct a boat; travel to a new generated island; carry over upgrades.

**Why next:** Endgame progression after one island is stable.

**Bevy patterns used:** `SubStates`, resource carry-over, world unloading, seeded regeneration.

**Result at the end of this milestone:** Build a boat, sail to a new island.

---

## Milestone 17: Audio, Particles, and Screenshake

**Goal:** Music, sound effects, particle effects, screenshake.

**Why next:** Final polish.

**Bevy patterns used:** `AudioPlayer`, `PlaybackSettings`, particle spawning, timers.

**Result at the end of this milestone:** The game feels finished.

---

## Milestone 18: Co-op (Optional)

**Goal:** Local or online co-op with two monarchs.

**Why optional:** Large scope; likely a separate mini-series.

**Bevy patterns used:** input mapping, shared camera, networking.

---

## Milestone 19: Spatial Indexing (Optional)

**Goal:** Implement a simple spatial index (for example, a 1D grid along the x-axis) so Greed and archers can find nearby targets without scanning every entity.

**Why optional:** The core game has too few entities to need this. This milestone is purely a demonstration: *“You can do X in Bevy; it is not useful at this scale, but it will be in a much larger game.”* Tutorials are where this kind of overengineering is acceptable, as long as it is framed as a demonstration rather than a requirement.

**Bevy patterns used:** custom `Resource` for the index, systems that rebuild the index from queries, lookup systems, `SystemSet` ordering.

**Result at the end of this milestone:** Targeting uses the spatial index. Performance is unchanged at tutorial scale, but the pattern is shown.

---

## Developer Cheats

During development we may add temporary cheats to keep iteration fast: infinite gold, accelerated day/night, instant build, etc. These are not milestones; they are added as needed and removed or hidden before release.
