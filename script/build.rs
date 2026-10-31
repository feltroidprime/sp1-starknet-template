use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    // Le chemin interne et le compilateur restent identiques entre le poste local et la CI.
    std::env::set_var("SP1_DOCKER_IMAGE", "ghcr.io/succinctlabs/sp1@sha256:9363a70004271de441e863612c7aefbbc5c36f371f111cd8abb1672fa5ea99f1");
    build_program_with_args(
        "../program",
        BuildArgs {
            docker: true,
            locked: true,
            ..Default::default()
        },
    );
}
