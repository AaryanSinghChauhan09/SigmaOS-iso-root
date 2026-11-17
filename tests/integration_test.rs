// Integration test for SigmaOS-iso-root
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_iso_root as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
