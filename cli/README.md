# Kortex CLI

This directory contains the OpenAPI specification for the Kortex CLI.

## OpenAPI Specification

The `openapi.yaml` file describes the commands available in the Kortex CLI:

- **Each path** in the specification corresponds to a CLI command
- **Response formats** define the structure of the command output when using the `-o json` flag

### Example

For a path `/list` with a GET operation, the corresponding CLI command would be:

```bash
kortex-cli list -o json
```

The JSON output format is defined by the response schema in the OpenAPI specification.
