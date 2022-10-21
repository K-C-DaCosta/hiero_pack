#[allow(unused_imports)]
use super::*;

#[test]
fn sanity() {
    let text = "in fa=\"An Mo\" si=32 bo=0";

    let t1 = HieroTokenizer::tokenize_line(text);
    assert_eq!(
        t1,
        vec![
            HieroToken::EntryName("in"),
            HieroToken::Pair {
                key: "fa",
                val: "An Mo"
            },
            HieroToken::Pair {
                key: "si",
                val: "32"
            },
            HieroToken::Pair {
                key: "bo",
                val: "0"
            },
        ]
    );
}

#[test]
fn harder() {
    let alt_text = "info face=\"Andale Mono\" size=32 bold=0 italic=0 charset=\"\" unicode=0 stretchH=100 smooth=1 aa=1 padding=1,1,1,1 spacing=-2,-2";
    let t2 = HieroTokenizer::tokenize_line(alt_text);
    assert_eq!(
        t2,
        vec![
            HieroToken::EntryName("info"),
            HieroToken::Pair {
                key: "face",
                val: "Andale Mono"
            },
            HieroToken::Pair {
                key: "size",
                val: "32"
            },
            HieroToken::Pair {
                key: "bold",
                val: "0"
            },
            HieroToken::Pair {
                key: "italic",
                val: "0"
            },
            HieroToken::Pair {
                key: "charset",
                val: ""
            },
            HieroToken::Pair {
                key: "unicode",
                val: "0"
            },
            HieroToken::Pair {
                key: "stretchH",
                val: "100"
            },
            HieroToken::Pair {
                key: "smooth",
                val: "1"
            },
            HieroToken::Pair {
                key: "aa",
                val: "1"
            },
            HieroToken::Pair {
                key: "padding",
                val: "1,1,1,1"
            },
            HieroToken::Pair {
                key: "spacing",
                val: "-2,-2"
            },
        ]
    );
}