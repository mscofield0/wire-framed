use wire_framed::prelude::*;

#[derive(Debug, Decoding, PartialEq, Eq)]
pub struct Foo {
    pub a: u32,
    pub b: u16,
    pub c: String,
    pub d: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use wire_framed::bytes::Bytes;

    use super::*;

    #[test]
    fn test1() {
        let frame = Bytes::from_static(&[
            0, 0, 0, 1, // a
            0, 2, // b
            0, 0, 0, 5, // c
            104, 101, 108, 108, 111, // c
            0, 0, 0, 4, // d
            1, 2, 3, 4, // d
        ]);
        let foo = Foo::from_frame(frame).unwrap();

        let result = Foo {
            a: 1,
            b: 2,
            c: "hello".to_string(),
            d: vec![1, 2, 3, 4],
        };

        assert_eq!(foo, result);
    }

    #[test]
    fn tuple_test() {
        let frame = Bytes::from_static(&[
            0, 0, 0, 1, // 0
            0, 0, 0, 2, // 1
        ]);
        let (a, b): (u32, u32) = FromFrame::from_frame(frame).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }
}