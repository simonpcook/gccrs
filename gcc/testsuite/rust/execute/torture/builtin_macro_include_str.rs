// { dg-output "hello, include!\n" }

macro_rules! include_str {
  () => {{}};
}

extern "C" {
    fn printf(fmt: *const i8, ...);
}

fn print(s: &str) {
  printf("%s" as *const str as *const i8, s as *const str as *const i8);
}


fn main() -> i32 {
  // include_str! (and include_bytes!) allow for an optional trailing comma.
  let my_str = include_str! ("include.txt",);

  print (my_str);

  0
}
