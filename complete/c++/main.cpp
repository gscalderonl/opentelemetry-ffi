#include <opentelemetry/trace/tracer.h>
#include <opentelemetry/sdk/trace/simple_processor.h>
#include <opentelemetry/exporters/otlp/otlp_exporter.h>
#include <iostream>

extern "C" {
    void run_trace();
}

int main() {
    // Initialize the OpenTelemetry trace provider
    auto exporter = std::make_shared<sdktrace::OtlpExporter>(sdktrace::OtlpExporterOptions{});
    auto processor = std::make_shared<sdktrace::SimpleSpanProcessor>(exporter);
    auto provider = nostd::shared_ptr<opentelemetry::trace::TracerProvider>(new sdktrace::TracerProvider(processor));
    opentelemetry::trace::Provider::SetTracerProvider(provider);

    // Call the Rust function to run the trace
    run_trace();

    // Wait for a key press before exiting
    std::cout << "Press any key to exit..." << std::endl;
    std::cin.get();

    return 0;
}
