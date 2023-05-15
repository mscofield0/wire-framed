use wire_framed::prelude::*;

#[derive(Debug, Encoding, Clone, PartialEq, Eq)]
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
        let foo = Test::Baz("John".to_string());

        assert_eq!(foo.size_hint(), 8);

        let frame = foo.into_frame();
        let result = Bytes::from_static(&[
            2, // Baz
            0, 0, 0, 4, // length
            74, 111, 104, 110, // John
        ]);

        assert_eq!(frame, result);
    }
}