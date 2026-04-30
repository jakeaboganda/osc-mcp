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

- **Multi-version support**: OpenSCENARIO 1.0, 1.1, and 1.2
- **Fail-fast validation**: Immediate error feedback with verbose messages
- **Read-only catalog support**: Load entities from external catalogs
- **OpenDRIVE integration**: Validate road positions against loaded road networks
- **MCP server**: Stateful session management for AI agent workflows

## Development Status

🚧 **Under active development** - Core library and MCP server implementation in progress.

## License

MIT
