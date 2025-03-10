const GIT_COMMIT: &&str = &"GIT_COMMIT";

fn main() {
    // Rebuild if the tests have changed
    println!("cargo:rerun-if-changed=tests");

    // Only use build_data if the environment variable isn't set
    // The environment variable is always set when working via Nix
    if std::env::var(GIT_COMMIT).is_err() {
        build_data::set_GIT_COMMIT();
        build_data::set_GIT_DIRTY();
        build_data::no_debug_rebuilds();
    }
}
