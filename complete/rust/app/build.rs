fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    tonic_build::compile_protos("proto/opentelemetry/proto/collector/trace/v1/trace_service.proto").unwrap();
}
