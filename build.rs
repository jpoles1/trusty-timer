extern crate embed_resource;

fn main() {
    #[cfg(build = "release")]
    {
        embed_resource::compile("trusty-timer-manifest.rc");
    }
}