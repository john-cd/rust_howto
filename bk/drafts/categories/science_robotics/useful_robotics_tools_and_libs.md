# Useful tools and libraries for robotics

{{#include useful_robotics_tools_and_libs.incl.md}}

## Use computer vision in your robot {#opencv}

[![opencv][c-opencv-badge]][c-opencv]{{hi:opencv}}
[![opencv-crates.io][c-opencv-crates.io-badge]][c-opencv-crates.io]
[![opencv-github][c-opencv-github-badge]][c-opencv-github]
[![opencv-lib.rs][c-opencv-lib.rs-badge]][c-opencv-lib.rs]
[![cat-science::robotics][cat-science::robotics-badge]][cat-science::robotics]{{hi:Robotics}}

[OpenCV][c-opencv-website]{{hi:opencv}}⮳

[![opencv-example][c-opencv-example-badge]][c-opencv-example]

## Zero Overhead Pub/Sub/Query Protocol with `zenoh` {#zenoh}

[![zenoh][c-zenoh-badge]][c-zenoh]{{hi:zenoh}}
[![zenoh-crates.io][c-zenoh-crates.io-badge]][c-zenoh-crates.io]
[![zenoh-github][c-zenoh-github-badge]][c-zenoh-github]
[![zenoh-lib.rs][c-zenoh-lib.rs-badge]][c-zenoh-lib.rs]

[Zenoh][c-zenoh-website]{{hi:zenoh}}⮳ is a zero-overhead Pub/Sub/Query protocol. Zenoh (pronounced as /zeno/) unifies data in motion, data at rest and computations. It blends traditional pub/sub with geo-distributed storages, queries and computations, while retaining time and space efficiency.

Zenoh is a great tool for data storage, query, and computations over geographically distributed systems.

Zenoh deals with keys/values where each key is a path and is associated to a value. A key looks like just a Unix file system path, such as `myhome/kitchen/temperature`. The value can be defined with different encodings (string, [JSON][p-json], raw bytes buffers).

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/zenoh.rs:example}}
```

## Open Rust Robotics {#open-rust-robotics}

[![openrr][c-openrr-badge]][c-openrr]{{hi:openrr}}
[![openrr-crates.io][c-openrr-crates.io-badge]][c-openrr-crates.io]
[![openrr-github][c-openrr-github-badge]][c-openrr-github]
[![openrr-lib.rs][c-openrr-lib.rs-badge]][c-openrr-lib.rs]

[`openrr`][c-openrr]⮳{{hi:openrr}}

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/openrr.rs:example}}
```

## Create a behavior tree with `bonsai-bt` {#bonsai-bt}

[![bonsai-bt][c-bonsai_bt-badge]][c-bonsai_bt]{{hi:bonsai-bt}}
[![bonsai-bt-crates.io][c-bonsai_bt-crates.io-badge]][c-bonsai_bt-crates.io]
[![bonsai-bt-github][c-bonsai_bt-github-badge]][c-bonsai_bt-github]
[![bonsai-bt-lib.rs][c-bonsai_bt-lib.rs-badge]][c-bonsai_bt-lib.rs]

[`bonsai-bt`][c-bonsai_bt]⮳{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][behavior-tree-wikipedia] (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/bonsai_bt.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[useful_robotics_tools_and_libs: locate libs, organize, write (P2)](https://github.com/john-cd/rust_howto/issues/479)

[P2 link to `opencv` example under computer vision](https://github.com/john-cd/rust_howto/issues/960)

[behavior-tree-wikipedia]:https://en.wikipedia.org/wiki/Behavior_tree_(artificial_intelligence%2C_robotics_and_control)

zenoh in [network programming][p-network-programming]?
</div>
