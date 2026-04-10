# Kaiden Workspace Configuration Type

[![npm version](https://img.shields.io/npm/v/@kortex-hub/kortex-workspace-configuration.svg)](https://www.npmjs.com/package/@kortex-hub/kortex-workspace-configuration)
[![Apache-2.0](https://shields.io/badge/license-Apache%202-yellow.svg)](https://opensource.org/license/apache-2-0)

TypeScript type definitions for the Kaiden Workspace Configuration.

## Installation

```bash
npm install @kortex-hub/kortex-workspace-configuration
```

Or with your preferred package manager:

```bash
# pnpm
pnpm add @kortex-hub/kortex-workspace-configuration

# yarn  
yarn add @kortex-hub/kortex-workspace-configuration

# bun
bun add @kortex-hub/kortex-workspace-configuration
```

## Usage

```typescript
import type { components } from '@kortex-hub/kortex-workspace-configuration';

type WorkspaceConfiguration = components['schemas']['WorkspaceConfiguration'];
```

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
