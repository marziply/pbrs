pub fn indent(depth: u8) -> String {
  (0..depth).map(|_| "  ").collect()
}
