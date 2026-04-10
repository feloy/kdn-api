# Kaiden

This repository contains OpenAPI specifications and generated code for Kaiden components.

## Directory Structure

### [`cli/`](cli/)
Contains the OpenAPI specification for the Kaiden CLI (kdn) and generated type definitions.

- **`openapi.yaml`** - OpenAPI specification describing the commands available in the Kaiden CLI (kdn)
- **`go/`** - Go type definitions for the CLI client
- **`typescript/`** - TypeScript type definitions for the CLI client

Each path in the OpenAPI specification corresponds to a CLI command, and response formats define the structure of command output when using the `-o json` flag.

### [`workspace-configuration/`](workspace-configuration/)
Contains the OpenAPI specification for Kaiden Workspace configuration and generated type definitions.

- **`openapi.yaml`** - OpenAPI specification for workspace configuration
- **`go/`** - Go type definitions for the workspace configuration
- **`typescript/`** - TypeScript type definitions for the workspace configuration

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
