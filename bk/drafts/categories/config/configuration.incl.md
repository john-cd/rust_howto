<!-- TODO Reviewed ??-??-2026 -->

Layer configuration in a clear precedence order: built-in defaults, configuration files, environment variables, then command-line options. Deserialize the merged result into a typed struct so missing or invalid settings fail during startup.

[`config`][c~config~docs]↗{{hi:config}} supports layered sources and several file formats. [`figment`][c~figment~docs]↗{{hi:figment}} tracks the source and profile for values and errors, and is used by Rocket. [`confique`][c~confique~docs]↗{{hi:confique}} provides typed, Serde-based layered configuration. Use [`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} to load a development `.env` file into the process environment; production environments usually set variables directly.

| Recipe                                                                                        | Crates                                          | Categories                                    |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------- | --------------------------------------------- |
| [Load Typed Settings from Multiple Sources with `config`][ex~config~config]                   | [![config][c~config~docs~badge]][c~config~docs] | [![cat~config][cat~config~badge]][cat~config] |
| [Load Configuration Settings into a Singleton][ex~config~load_configuration_into_a_singleton] | [![config][c~config~docs~badge]][c~config~docs] | [![cat~config][cat~config~badge]][cat~config] |
| [Test Configuration-loading Methods][ex~config~test_configuration_loading_methods]            | [![config][c~config~docs~badge]][c~config~docs] | [![cat~config][cat~config~badge]][cat~config] |
| [Load a TOML Configuration File with `confy`][ex~config~confy]                                | [![confy][c~confy~docs~badge]][c~confy~docs]    | [![cat~config][cat~config~badge]][cat~config] |
