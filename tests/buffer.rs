use rupl::buffer::{Buffer, BufferError, CursorBuffer, Direction};

#[test]
fn buffer_basic() {
    let mut buf = Buffer::new();

    buf.insert(0, &['a', 'b', 'c']).unwrap();
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_bytes(), vec![97, 98, 99]);

    buf.remove(2, 1).unwrap();
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.as_bytes(), vec![97, 98]);

    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.as_bytes(), vec![]);
}

#[test]
fn buffer_remove() {
    let mut buf = Buffer::new();

    buf.insert(0, &['a', 'b', 'c']).unwrap();

    buf.remove(2, 1).unwrap();
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.as_bytes(), vec![97, 98]);

    buf.remove_from_to(0, 2).unwrap();
    assert!(buf.is_empty());
}

#[test]
fn buffer_insert_invalid_index() {
    let mut buf = Buffer::new();

    let res = buf.insert(1, &['a', 'b', 'c']);
    assert_eq!(res, Err(BufferError::InvalidStartIndex))
}

#[test]
fn buffer_remove_invalid_count() {
    let mut buf = Buffer::new();

    buf.insert(0, &['a', 'b', 'c']).unwrap();
    assert_eq!(
        buf.remove(1, 3),
        Err(BufferError::DeleteCountOverflow { at: 1, count: 3 })
    )
}

#[test]
fn cursor_buffer_basic() {
    let mut buf = CursorBuffer::new();

    buf.insert(&['a', 'b', 'c']).unwrap();
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.get_pos(), 3);

    buf.remove_one(Direction::Left).unwrap();
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.get_pos(), 2);
    assert_eq!(buf.as_bytes(), vec![97, 98]);

    let moved = buf.move_left();
    assert_eq!(moved, true);

    buf.insert(&['x', 'y', 'z']).unwrap();
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.get_pos(), 4);
    assert_eq!(buf.as_bytes(), vec![97, 120, 121, 122, 98]);
}
