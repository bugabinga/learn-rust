#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

      // str is copied on the stack
      let a_simple_text = "this is some text";
      let s_mine_now = a_simple_text;
      let i_wanna_too = a_simple_text;

      assert_eq!(s_mine_now, i_wanna_too);

      let heap_text = String::from("popo");
      let give_it = &heap_text;
      let me_too = &heap_text;

      assert_eq!(give_it, me_too);
    }
}
