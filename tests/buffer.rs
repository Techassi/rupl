use rupl::buffer::Buffer;

#[test]
fn test_buffer() {
    let mut buf = Buffer::new();

    buf.insert(0, &['a', 'b', 'c']).unwrap();
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_bytes(), vec![97, 98, 99]);

    buf.remove(2, 1);
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.as_bytes(), vec![97, 98]);

    buf.clear();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.as_bytes(), vec![]);
}
