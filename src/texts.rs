pub const AUTHORS: &'static str =
    color_print::cstr!("<s>Daniele Olmisani</s> - <u>daniele.olmisani@gmail.com</u>");

pub const ABOUT: &'static str =
    color_print::cstr!("Display a customizable date tag (e.g. <i>TEST_202404</i>, <i>2024-04-03_rel</i>, <i>2024.04.03</i>)");

pub const EXAMPLES: &'static str = color_print::cstr!(
    r#"<s><u>Examples</u></s>:
    $ <s>datetag</> 20220312 --offset 22 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ <s>datetag</> 20220312 -o 22 -p 'TEST_' -td
    TEST_20220403
"#
);
