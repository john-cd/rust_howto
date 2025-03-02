# Artificial Intelligence for Robotics

{{#include artificial_intelligence.incl.md}}

Behavior trees, decision-making, machine learning.

Consider `bonsai-bt` (Behavior Tree implementation) and `optimization-engine`, a pure Rust framework for embedded nonconvex optimization.

## Create a behavior tree with `bonsai-bt` {#bonsai-bt}

[![bonsai-bt][c-bonsai_bt-badge]][c-bonsai_bt]{{hi:bonsai-bt}}
[![bonsai-bt-crates.io][c-bonsai_bt-crates.io-badge]][c-bonsai_bt-crates.io]
[![bonsai-bt-github][c-bonsai_bt-github-badge]][c-bonsai_bt-github]
[![bonsai-bt-lib.rs][c-bonsai_bt-lib.rs-badge]][c-bonsai_bt-lib.rs]

[`bonsai-bt`][c-bonsai_bt]⮳{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][behavior-tree-wikipedia] (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/tests/bonsai_bt.rs:example}}
```

## Related Topics

- [[computer-vision | Computer Vision]]
- [[classical_machine_learning | Classical Machine Learning]]
- [[deep_learning | Deep Learning]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
[behavior-tree-wikipedia]:https://en.wikipedia.org/wiki/Behavior_tree_(artificial_intelligence%2C_robotics_and_control)

Rust Crates:

- `dora-rs`: Robotics framework for AI applications. Low usage.
- `optimization-engine`: Embedded optimization for robotics.

</div>
