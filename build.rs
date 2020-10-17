#[cfg(feature = "static")]
use actix_web_static_files::NpmBuild;
#[cfg(feature = "static")]
fn build_assets() {
    NpmBuild::new("./search")
        .executable("yarn")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("./search/dist")
        .to_resource_dir()
        .build()
        .unwrap();
}

#[cfg(not(feature = "static"))]
fn build_assets() {}
fn main() {
    build_assets()
}
