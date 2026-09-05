//! Fixture: every forbidden purity token, planted as if it were real stratum
//! 1 code. Scanned by `purity.rs`'s
//! `each_forbidden_token_planted_in_a_stratum_1_source_is_caught`; never
//! built.

fn reach_out() {
  let _ = std::fs::metadata(".");
  let _ = std::process::id();
  let _ = std::net::Ipv4Addr::LOCALHOST;
  let _ = std::os::unix::fs::PermissionsExt::mode;
  let _ = std::env::var("HOME");
  let _ = std::thread::current();
  let _ = std::io::stdout();
  let _ = std::time::SystemTime::now();
  let _ = std::time::Instant::now();
}
