# havok Classes Parser & Json Generator

Calculate C++ offsets, and output Havok Behavior Class information to json.

- Normal

  - Output dir: crates/hkx_class_generator/assets/classes

```shell
cargo xtask generate
```

- For nemesis json(i8..=i64, u8..=u64, pointer -> accepts string)

  - Output dir: crates/hkx_class_generator/assets/nemesis/classes

```shell
cargo xtask generate --mode nemesis
```

- On CI

  It is also possible to fork and run CI to automatically collect necessary information and generate json.

  - Artifact: hk_2010.2.0-r1 json(Normal & Nemesis).zip
