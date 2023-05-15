use wire_framed::prelude::*;

#[derive(Encoding)]
pub struct Foo {
    pub a: u32,
    pub b: u16,
    pub c: String,
    pub d: Vec<u8>,
}

#[derive(Encoding)]
pub struct Bar;

#[cfg(test)]
mod tests {
    use wire_framed::bytes::Bytes;

    use super::*;

    #[test]
    fn test1() {
        let foo = Foo {
            a: 1,
            b: 2,
            c: "hello".to_string(),
            d: vec![1, 2, 3, 4],
        };

        let frame = foo.into_frame();
        let result = Bytes::from_static(&[
            0, 0, 0, 1, // a
            0, 2, // b
            0, 0, 0, 5, // c
            104, 101, 108, 108, 111, // c
            0, 0, 0, 4, // d
            1, 2, 3, 4, // d
        ]);

        assert_eq!(frame, result);
    }
}