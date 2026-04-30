# OpenSCENARIO MCP Server

Rust library and MCP server for AI agents to generate, validate, and export ASAM OpenSCENARIO test scenarios.

## Project Structure

```
osc-mcp/
├── Cargo.toml                    # Workspace configuration
├── openscenario/                 # Core library crate
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs               # Library entry point
│   └── tests/                    # Integration tests
└── openscenario-mcp/             # MCP server crate
    ├── Cargo.toml
    └── src/
        └── main.rs               # Server entry point
```

## Features

- Multi-version support (OpenSCENARIO 1.0, 1.1, 1.2)
- Fail-fast validation with verbose errors
- Read-only catalog support
- OpenDRIVE integration for position validation
- MCP server for AI agent workflows

## License

MIT
