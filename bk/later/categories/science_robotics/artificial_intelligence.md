# Artificial Intelligence for Robotics

{{#include artificial_intelligence.incl.md}}

Behavior trees, decision-making, machine learning.

Consider [`bonsai-bt`][c~bonsai_bt~docs]⮳{{hi:bonsai-bt}} (Behavior Tree implementation) and `optimization-engine`, a pure Rust framework for embedded nonconvex optimization.

## Create a Behavior Tree with `bonsai-bt` {#bonsai-bt}

[![bonsai-bt][c~bonsai_bt~docs~badge]][c~bonsai_bt~docs]{{hi:bonsai-bt}}
[![bonsai-bt~crates.io][c~bonsai_bt~crates.io~badge]][c~bonsai_bt~crates.io]
[![bonsai-bt~github][c~bonsai_bt~github~badge]][c~bonsai_bt~github]
[![bonsai-bt~lib.rs][c~bonsai_bt~lib.rs~badge]][c~bonsai_bt~lib.rs]

[`bonsai-bt`][c~bonsai_bt~docs]⮳{{hi:bonsai-bt}} is a Rust implementation of behavior trees. A [Behavior Tree (Wikipedia)][behavior-tree~wikipedia] (BT) is a data structure in which we can set the rules of how certain behavior's can occur, and the order in which they would execute. BTs are a very efficient way of creating complex systems that are both modular and reactive. These properties are crucial in many applications, which has led to the spread of BT from computer game programming to many branches of AI and Robotics.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/artificial_intelligence/bonsai_bt.rs:example}}
```

## Related Topics {#skip}

- [[computer-vision | Computer Vision]].
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1209)
[behavior-tree~wikipedia]:https://en.wikipedia.org/wiki/Behavior_tree_(artificial_intelligence%2C_robotics_and_control)

Rust Crates:

- `dora-rs`: Robotics framework for AI applications. Low usage.
- `optimization-engine`: [Embedded][p~embedded] optimization for robotics.

</div>
