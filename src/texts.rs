pub const AUTHORS: &'static str =
    color_print::cstr!("<s>Daniele Olmisani</s> - <u>daniele.olmisani@gmail.com</u>");

pub const ABOUT: &'static str =
    color_print::cstr!("Display a customizable date tag (e.g. <i>TEST_202404</i>, <i>2024-04-03_rel</i>, <i>2024.04.03</i>)");

pub const EXAMPLES: &'static str = color_print::cstr!(
    r#"<s><u>Examples</u></s>:
    $ <s>datetag</> 20240312 --offset 22 --prefix 'TEST_' --tag-type daily
    TEST_20240403

    $ <s>datetag</> 20240312 -o 22 -p 'TEST_' -td
    TEST_20240403

    $ <s>datetag</> 20240312 -o 2 -r3 -td -s dot
    2024.03.12
    2024.03.14
    2024.03.16
"#
);

pub const NOTES: &'static str = color_print::cstr!(
    r#"<s><u>Notes</u></s>:
    Argument '--format' use string format from:
    https://docs.rs/chrono/latest/chrono/format/strftime/index.html
"#
);

