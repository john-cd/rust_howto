<!-- TODO Reviewed ??-??-2026 -->

# 2D and 3D Physics Engines

{{#include physics_engines.incl.md}}

## Rapier {#rapier}

[![rapier][rapier~badge]][rapier~website]{{hi:rapier}}

Rapier provides 2D and 3D rigid-body dynamics, joints, collision detection, and geometric queries. Use `rapier2d` or `rapier3d` directly when your application owns the simulation loop and does not need Bevy integration.

The following standalone 2D example steps a falling ball on a fixed ground. It uses the pinned `rapier2d` crate without a GUI backend and compiles on Windows.

```rust,editable
{{#include ../../../crates/cats/simulation/examples/physics_engines/rapier2d.rs:example}}
```

## Parry {#parry}

[![parry3d][c~parry3d~docs~badge]][c~parry3d~docs]{{hi:parry}}

Parry provides collision detection and geometric queries such as ray casts, shape casts, and distance checks. Use `parry2d` or `parry3d` when you need those queries without a rigid-body simulation.

## Avian {#avian}

[![avian2d][c~avian2d~docs~badge]][c~avian2d~docs]{{hi:avian}}

Avian is an ECS-driven 2D and 3D physics engine designed for Bevy. Choose `avian2d` or `avian3d` when you want physics systems and components to fit into a Bevy application.

## `bevy_rapier` {#bevy-rapier}

[![bevy_rapier2d][c~bevy_rapier2d~docs~badge]][c~bevy_rapier2d~docs]{{hi:bevy_rapier}}

`bevy_rapier2d` and `bevy_rapier3d` integrate Rapier physics into Bevy applications. Choose the plugin for an existing Bevy project that uses Rapier, and check that the plugin release supports your Bevy version.

## See Also {#see-also .skip}

- [Physics][are-we-game-yet?-physics~website]↗.

## Related Topics {#related-topics .skip}

- [[game-development | Game Development]].
- [[graphics | Graphics]].
- [[physics | Physics]].
- [[visualization | Visualization]].
- [Simulation and Visualization][p~robotics-simulation] shows a headless Rapier example for a robot chassis.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
