<!-- TODO Reviewed ??-??-2026 -->

# clean

A very simple tool to clean `temp` folders before and after testing.
Keeps any hidden file (such as `.gitkeep`) in these folders.

```bash
clean <directory>
```

- Every directory named `temp` below `<directory>` is emptied: visible files and all subfolders
  are removed.
- Symbolic links and reparse points are never followed; a link inside `temp` is removed as a link
  and its target is left untouched. Nothing outside `<directory>` is modified.
- Traversal finishes before anything is deleted. A missing root or an unreadable directory prints an
  error with the path, exits with a nonzero status and deletes nothing.
