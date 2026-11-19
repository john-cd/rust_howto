<!-- TODO Reviewed ??-??-2026. -->

# In the age of AI

A large language model (LLM) can write Rust code and explain it. An agent can also inspect your repository, edit files, and run checks. Both are more useful when you can name the approaches worth considering, explain what your project needs, and recognize when the proposed solution falls short.

Use this book to build that judgment. Browse the topics to learn what Rust and its ecosystem make possible. Read a recipe when you need to see an approach in action. You do not need to memorize every crate or API. Knowing that a tool exists, what problem it solves, and where to look up its limits is often enough to ask a much better question.

## Use the book before you ask

Start with the part of the book that matches your problem. Look at the language and standard library chapters for the primitives you already have. Browse the category chapters for reusable crates and patterns. Use those recipes to build a shortlist, then check the documentation for the versions and features your project actually uses.

Turn what you find into constraints the model can act on. Name the behavior you need, the platforms and Rust versions you support, and the interfaces you need to preserve. Ask which options fit, what each one costs, and what evidence would change the recommendation. For an agent, also say which files it can change, which commands it may run, and which decisions it should bring back to you.

## A worked example

Suppose a service accepts work faster than its workers can finish it, and memory grows under load. The book's [Message Passing and Channels](../categories/concurrency/message_passing.md) and [Async Channels](../categories/asynchronous/async_channels.md) chapters give you the vocabulary to investigate. A bounded channel caps the number of items waiting in the queue. You still have to decide what producers do when the queue fills and what happens to accepted work during shutdown.

If the project already uses Tokio and callers can retry, you could give an agent this brief.

> Trace the current path from submission to completion. Compare using a bounded channel in our existing runtime with keeping the current queue and enforcing a limit at submission. When capacity is exhausted, return a retryable error promptly. On graceful shutdown, finish accepted work. Explain how each option meets those requirements before editing. Flag any new dependency or public API change for review.

That brief gives you something concrete to review. A bounded channel can still leave memory growing if every producer waits in its own spawned task while holding a job. Trace where jobs are allocated, who owns them, and how many can be in flight. Check what happens when the queue fills, the receiver closes, or shutdown starts with work pending. Those questions matter even when the code compiles.

## Review the choices and the evidence

- **Read the diff against the requested behavior.** Check error paths, resource limits, and changes to public interfaces. Revisit [Ownership and Borrowing](../language/ownership_and_borrowing.md) or [Recoverable Error Handling](../standard-library/recoverable_error_handling.md) when you need to understand why a change clones data, discards an error, or changes who owns a resource.
- **Review dependency changes.** Inspect the manifest and lockfile. Check that each API exists in the selected version and that its feature flags and platform requirements fit the project. For a new crate, weigh what it saves against its maintenance, license, and transitive dependencies. The [Dependencies](../code-organization/dependencies.md) chapter shows where these choices are expressed.
- **Check what the tests prove.** Ask the agent to report the commands it ran, their results, and any target or feature combination it skipped. Read the assertions as closely as the implementation. For the queue, a test should fill it deliberately and observe the retryable error. A separate test should establish that graceful shutdown completes accepted work. See [Testing](../categories/development-tools_testing/testing.md) for ways to put those behaviors under test.

A model may recall more APIs than you do and produce an implementation faster. You can still make the wiser choice because you know who will rely on the software, which failures are acceptable, and what your team can maintain. The book helps you notice the choices. Your job is to decide which ones serve the project and verify that the code follows through.
