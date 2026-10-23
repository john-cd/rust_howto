<!-- TODO Reviewed ??-??-2026 -->
# Cancelable Task Example

This example demonstrates a worker thread with a bounded task queue and progress responses.
Each queued task receives its own cooperative cancellation token, so cancelling one task
cannot consume another task's request. Queue saturation and attempts to cancel an unknown
task return errors. Stopping the worker requests cancellation for all registered tasks,
signals shutdown, joins the thread, and lets the caller drain its progress responses.
