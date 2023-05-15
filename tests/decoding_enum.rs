use wire_framed::prelude::*;

#[derive(Debug, Decoding, Clone, PartialEq, Eq)]
pub enum Test {
    Foo(u32),
    Bar(u16),
    Baz(String),
}

#[cfg(test)]
mod tests {
    use wire_framed::bytes::Bytes;

    use super::*;

    #[test]
    fn test1() {
        let frame = Bytes::from_static(&[
            2, // Baz
            0, 0, 0, 4, // length
            74, 111, 104, 110, // John
        ]);
        let foo = Test::from_frame(frame).unwrap();

        let result = Test::Baz("John".to_string());

        assert_eq!(foo, result);
    }
}