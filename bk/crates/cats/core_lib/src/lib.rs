**Title: Re-export Templates in `core_lib` for Improved Usability**

**Issue:**
In the current implementation of `bk/crates/cats/core_lib/src/lib.rs`, several templates are defined but not easily accessible for external use. This restriction complicates the workflow for developers who need to utilize these templates in their applications or extensions.

**Proposed Solution:**
To enhance usability and facilitate better integration with other modules, it is essential to re-export these templates from `core_lib`. This will allow them to be readily available for users without the need for deep-diving into the source files.

**Benefits:**
- **Simplicity:** It simplifies the import process for developers utilizing the `core_lib` templates.
- **Efficiency:** Saves time by reducing the complexity of accessing templates directly from their definitions.
- **Consistency:** Promotes consistent usage across different parts of the codebase and external projects.

**Link to File:**
You can view and reference the relevant file here: [lib.rs](https://github.com/john-cd/rust_howto/blob/main/bk/crates/cats/core_lib/src/lib.rs)

**Next Steps:**
- Review the templates currently defined in the file.
- Discuss the re-export strategy within the team.
- Implement the changes and update the documentation accordingly.