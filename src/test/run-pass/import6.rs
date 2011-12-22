import bar::baz;
import foo::zed;
mod foo {
    mod zed {
        fn baz() { #debug("baz"); }
    }
}
mod bar {
    import zed::baz;
    export baz;
}
fn main() { baz(); }
