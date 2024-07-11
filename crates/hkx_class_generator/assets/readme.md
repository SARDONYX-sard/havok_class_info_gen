# hkxcmd Report output directory

```shell
hkxcmd Report ./hkxcmd_help
```

## hkx2lib

The x64 offset and size are not computable due to the presence of hidden data that is not reported in the hkxcmd report.

Therefore, x64 offset and size data are obtained from HKX2Library using regular expressions.
It is done by the following command.

- [HKX2Library](https://github.com/ret2end/HKX2Library)

```shell
git clone https://github.com/ret2end/HKX2Library.git
python3 ./get_rpt.py
```

## classes

Reflection data in `hk_2010.2.0-r1`, but some data hidden by multiple inheritance of parent classes etc. do not exist.

```json
{
  "name": "hkpCollisionFilter",
  "parent": ["hkReferencedObject", "hkpCollidableCollidableFilter", "hkpShapeCollectionFilter", "hkpRayShapeCollectionFilter", "hkpRayCollidableFilter"]
},
{
  "name": "hkpShapeCollection",
  "parent": ["hkpShape", "hkpShapeContainer"]
},
{
  "name": "hkpConvexListShape",
  "parent": ["hkpConvexShape", "hkpShapeContainer"]
},
{
  "name": "hkpRemoveTerminalsMoppModifier",
  "parent": ["hkReferencedObject", "hkpMoppModifier"] // NOTE: "hkpMoppModifier" is not exist in reflection data!
},
{
  "name": "hkpTriggerVolume",
  "parents": ["hkReferencedObject", "hkpContactListener", "hkpWorldPostSimulationListener", "hkpEntityListener"]
   // NOTE: "hkpContactListener", "hkpWorldPostSimulationListener", "hkpEntityListener" is not exist in reflection data!
}
```

```rust
// Unknown padding
"BSRagdollContactListenerModifier" => class_info.members[0].offset_x86_64 = 88, // char \_unknown_padding[4];(+4) before 0th field

// Offset by field that does not appear in reflection data
"hkpWorld" => class_info.members[43].offset_x86_64 = 1008, // struct hkpWorldDynamicsStepInfo m_dynamicsStepInfo;(+328)
```

Also, there is one place where the signature of Class is different between hkxcmd and HKX2Library.(Reflection data is adopted from hkxcmd.)

hkpCompressedMeshShape

- hkxcmd: 0xa62d5e6e
- HKX2Library: 0xe3d1dba
