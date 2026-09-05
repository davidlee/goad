//! The one case that needs a real, if headless, Slint event loop: item 14e.
//!
//! A separate `[[test]]` target from `renderer` (design.md §9's placement
//! table, `:389-398`) because
//! `i_slint_backend_testing::init_integration_test_with_mock_time` "can only
//! be called once per process" — every other item in this crate's validation
//! runs `init_no_event_loop()` instead, which each test thread may call for
//! itself, and mixing the two backends into one binary is not an option.
#[cfg(test)]
mod closing;
