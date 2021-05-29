std::env;
std::fs;

 main() -> Result<(), Box<dyn std::error::Error>> {
    // The docs.rs build locks down write permissions and causes codegen to fail. Skip it since
    // it's not really needed rustdoc anyway.
    env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    protos = [
        (["protos/google/pubsub/v1/pubsub.proto"], "src/pubsub/api"),
        (
            ["protos/google/datastore/v1/datastore.proto"],
            "src/datastore/api",
        ),
        (
            ["protos/google/cloud/vision/v1/image_annotator.proto"],
            "src/vision/api",
        ),
    ];

     (proto_files, out_dir) in protos.iter() {
        fs::create_dir_all(&out_dir)?;

        tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .format(true)
            .out_dir(&out_dir)
            .compile(proto_files, &["protos"])?;

         file in proto_files {
            println!("cargo:return-if-changed={}", &file);
        }
    }

    Ok(())
}
