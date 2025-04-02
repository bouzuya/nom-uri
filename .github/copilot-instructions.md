# Copilot Instructions

- Only `IResult` and `Parser` can be used from the `nom` crate.
- Always use qualified imports for nom functions (e.g., `nom::character::satisfy` instead of importing `satisfy` directly).
- Follow RFC 3986 specifications for URI parsing.
- Include section references to RFC 3986 in comments for each parser component.
- Write comprehensive unit tests for each parser function.
- Maintain error handling using nom's error types.
- Use a modular approach by splitting different URI components into separate modules.
- Include doc comments for all public functions explaining their purpose.
