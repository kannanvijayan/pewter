use crate::types::{
  RecordType,
  DataType,
  DataTypeSpecialize,
  DataTypeSpecializeRef,
  DataTypeSpecializeMut,
  RecordTypeFieldSpecifier,
  record_info_for_type,
};

#[test]
fn test_point_record() {
  let record_info = record_info_for_type::<Point>();
  let pt = Point { x: 99, y: 33 };
  let mut bytes = (0 .. record_info.size())
    .map(|_| 0)
    .collect::<Vec<_>>();
  record_info.write_value(&pt, &mut bytes);
  let mut pt2 = Point::default();
  record_info.read_value(&mut pt2, &bytes);
  assert!(pt == pt2);
  assert!(&bytes == &[99, 0, 0, 0, 33, 0, 0, 0],
    "Unexpected bytes: {:?}", &bytes);
}

#[test]
fn test_rectangle_record() {
  let record_info = record_info_for_type::<Rect>();
  let rect = Rect {
    top_left: Point { x: 99, y: 33 },
    size: [101, 99014],
  };
  let mut bytes = (0 .. record_info.size())
    .map(|_| 0)
    .collect::<Vec<_>>();
  record_info.write_value(&rect, &mut bytes);
  let mut rect2 = Rect::default();
  record_info.read_value(&mut rect2, &bytes);
  assert!(rect == rect2);
  assert!(&bytes == &[
    99, 0, 0, 0,
    33, 0, 0, 0,
    101, 0, 0, 0,
    (99014 & 0xff) as u8,
      ((99014 >> 8) & 0xff) as u8,
      ((99014 >> 16) & 0xff) as u8,
      0,
  ],
    "Unexpected bytes: {:?}", &bytes);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Point {
  x: u32,
  y: u32
}
impl DataType for Point {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.record::<Self>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.record(self)
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.record(self)
  }
}
impl RecordType for Point {
  fn specify_fields<FS: RecordTypeFieldSpecifier<Self>>(fs: &mut FS) {
    fs.prim_field("x", |r| r.x, |r, x| r.x = x);
    fs.prim_field("y", |r| r.y, |r, y| r.y = y);
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Rect {
  top_left: Point,
  size: [u32; 2],
}
impl DataType for Rect {
  fn specialize<S: DataTypeSpecialize>(spez: S) -> S::Result {
    spez.record::<Self>()
  }
  fn specialize_ref<S: DataTypeSpecializeRef>(&self, spez: S) {
    spez.record(self)
  }
  fn specialize_mut<S: DataTypeSpecializeMut>(&mut self, spez: S) {
    spez.record(self)
  }
}
impl RecordType for Rect {
  fn specify_fields<FS: RecordTypeFieldSpecifier<Self>>(fs: &mut FS) {
    fs.record_field("top_left", |r| r.top_left, |r, tl| r.top_left = tl);
    fs.prim_field("size", |r| r.size, |r, sz| r.size = sz);
  }
}