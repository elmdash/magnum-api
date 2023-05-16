# Magnum API

The goal of this library is to generate "very good" [JSON Schemas](https://json-schema.org/) for Rust types including special attention for enums. The [OpenAPI 3.1.0](https://spec.openapis.org/oas/v3.1.0) specification and the corresponding [JSON Schema](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-00) documents are particularly loose when it comes to interpreting certain type mechanics available in Rust. Many of the available tools for working with JSON schemas and rendering Open API documents also struggle to produce reliable documentation for types derived from Rust types. The [schemars crate](https://github.com/GREsau/schemars) depends on [serde](https://github.com/serde-rs/serde) to generate all types and is dependent on their enum mechanics as well.

### The problem with enums

The serde crate provides multiple ways to express enums in JSON format. Newer OpenAPI versions also utilize a "discriminator" mechanic to help identify which variant is used, but it's not well supported. The schemars library is aimed at producing JSON schema output, but the discriminator is an OpenAPI add-on to the JSON Schema syntax which pushes it just beyond the reach of a general-purpose JSON schema library. Still, the discriminator provides just one way of expressing enum types.

### Dedicted to OpenAPI 3.x

Being focused on the OpenAPI specification allows us to make choices about JSON Schema representations that better suit an API specification with full support for documentation. There are many ways this can be done