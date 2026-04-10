# kdn

This directory contains the OpenAPI specification for the Kaiden CLI (kdn).

## OpenAPI Specification

The `openapi.yaml` file describes the commands available in the Kaiden CLI (kdn):

- **Each path** in the specification corresponds to a CLI command
- **Response formats** define the structure of the command output when using the `-o json` flag
- **Input parameters** are not defined in the specification - only the output formats are documented

### Example

For a path `/list` with a GET operation, the corresponding CLI command would be:

```bash
kdn list -o json
```

The JSON output format is defined by the response schema in the OpenAPI specification.

## Scenarios

### Workspace Management from UI

This scenario demonstrates how a UI or other machine can programmatically manage workspaces using the Kaiden CLI (kdn) with JSON output.

#### 1. List existing workspaces

```bash
$ kdn workspace list -o json
{
  "items": []
}
```

#### 2. Initialize a new workspace (minimal output)

```bash
$ kdn init --agent claude -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

#### 3. Initialize a new workspace (verbose output)

```bash
$ kdn init --agent claude -v -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea",
  "name": "workspace1",
  "project": "https://github.com/user/repo/",
  "agent": "claude",
  "model": "model1",
  "paths": {
    "source": "/home/user/workspace1",
    "configuration": "/home/user/workspace1/.kaiden"
  }
}
```

With explicit project:

```bash
$ kdn init --agent claude --project my-project -v -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea",
  "name": "workspace1",
  "project": "my-project",
  "agent": "claude",
  "model": "model1",
  "paths": {
    "source": "/home/user/workspace1",
    "configuration": "/home/user/workspace1/.kaiden"
  }
}
```

#### 4. List workspaces after initialization

```bash
$ kdn workspace list -o json
{
  "items": [
    {
      "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea",
      "name": "workspace1",
      "project": "https://github.com/user/repo/",
      "agent": "claude",
      "model": "model1",
      "paths": {
        "source": "/home/user/workspace1",
        "configuration": "/home/user/workspace1/.kaiden"
      }
    }
  ]
}
```

#### 5. Start a workspace

```bash
$ kdn workspace start 2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

Or using the shorthand command:

```bash
$ kdn start 2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

#### 6. Stop a workspace

```bash
$ kdn workspace stop 2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

Or using the shorthand command:

```bash
$ kdn stop 2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

#### 7. Remove a workspace

```bash
$ kdn workspace remove 2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea -o json
{
  "id": "2c5f16046476be368fcada501ac6cdc6bbd34ea80eb9ceb635530c0af64681ea"
}
```

#### Error Handling

##### Non-existent source directory

```bash
$ kdn init --agent claude -o json /tmp/not-found
{
  "error": "Error: sources directory does not exist: /tmp/not-found"
}
```

##### Workspace not found

```bash
$ kdn workspace remove unknown-id -o json
{
  "error": "Error: workspace not found: unknown-id"
}
```
