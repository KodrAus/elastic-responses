use inlinable_string::InlineString;
use string_cache::DefaultAtom as Interned;
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::io::Read;
use std::fmt;
use std::fs::File;

fn get_response() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut file = File::open("tests/samples/bulk_1000.json").unwrap();

    file.read_to_end(&mut buf).unwrap();
    
    buf
}

macro_rules! bench_all {
    ([$({$name:ident : $bulk_ty:ty}),*]) => (
        pub mod slice {
            use test::{Bencher, black_box};
            use elastic_responses::*;

            use super::*;

            $(
                slice_bench!($name : $bulk_ty);
            )*
        }

        pub mod read {
            use std::io::Cursor;
            use test::{Bencher, black_box};
            use elastic_responses::*;

            use super::*;

            $(
                read_bench!($name : $bulk_ty);
            )*
        }
    )
}

macro_rules! slice_bench {
    ($name:ident : $bulk_ty:ty) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let response = get_response();

            b.iter(|| {
                let bulk = parse::<$bulk_ty>().from_slice(200, &response).unwrap();

                black_box(bulk);
            });
        }
    )
}

macro_rules! read_bench {
    ($name:ident : $bulk_ty:ty) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let response = get_response();

            b.iter(|| {
                let read = Cursor::new(&response);
                let bulk = parse::<$bulk_ty>().from_reader(200, read).unwrap();

                black_box(bulk);
            });
        }
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Index {
    BulkTest
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    BulkTy
}

struct Inline(InlineString);

impl<'de> Deserialize<'de> for Inline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct InlineVisitor;

        impl<'de> Visitor<'de> for InlineVisitor {
            type Value = Inline;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where E: DeError
            {
                Ok(Inline(s.into()))
            }
        }

        deserializer.deserialize_any(InlineVisitor)
    }
}

bench_all!([
    { default_all : BulkResponse },
    { default_errors_only : BulkErrorsResponse },
    { enum_fields_all : BulkResponse<Index, Type> },
    { enum_fields_errors_only : BulkErrorsResponse<Index, Type> },
    { inline_fields_all : BulkResponse<Inline, Inline, Inline> },
    { inline_fields_errors_only : BulkErrorsResponse<Inline, Inline, Inline> },
    { interned_fields_all : BulkResponse<Interned, Interned, Interned> },
    { interned_fields_errors_only : BulkErrorsResponse<Interned, Interned, Interned> }
]);