#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`EnvironmentVariable`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the environment variable\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"secret\": {"]
#[doc = "      \"description\": \"Name of the secret containing the value (mutually exclusive with value)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"Value of the environment variable (mutually exclusive with secret)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentVariable {
    #[doc = "Name of the environment variable"]
    pub name: ::std::string::String,
    #[doc = "Name of the secret containing the value (mutually exclusive with value)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[doc = "Value of the environment variable (mutually exclusive with secret)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl EnvironmentVariable {
    pub fn builder() -> builder::EnvironmentVariable {
        Default::default()
    }
}
#[doc = "`McpCommand`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"description\": \"Arguments to pass to the command\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"command\": {"]
#[doc = "      \"description\": \"Command to run to start the MCP server\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"description\": \"Environment variables to set for the command\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the MCP server\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct McpCommand {
    #[doc = "Arguments to pass to the command"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub args: ::std::vec::Vec<::std::string::String>,
    #[doc = "Command to run to start the MCP server"]
    pub command: ::std::string::String,
    #[doc = "Environment variables to set for the command"]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Name of the MCP server"]
    pub name: ::std::string::String,
}
impl McpCommand {
    pub fn builder() -> builder::McpCommand {
        Default::default()
    }
}
#[doc = "`McpConfiguration`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"commands\": {"]
#[doc = "      \"description\": \"List of command-based MCP servers\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/McpCommand\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"servers\": {"]
#[doc = "      \"description\": \"List of URL-based MCP servers\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/McpServer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct McpConfiguration {
    #[doc = "List of command-based MCP servers"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub commands: ::std::vec::Vec<McpCommand>,
    #[doc = "List of URL-based MCP servers"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub servers: ::std::vec::Vec<McpServer>,
}
impl ::std::default::Default for McpConfiguration {
    fn default() -> Self {
        Self {
            commands: Default::default(),
            servers: Default::default(),
        }
    }
}
impl McpConfiguration {
    pub fn builder() -> builder::McpConfiguration {
        Default::default()
    }
}
#[doc = "`McpServer`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"headers\": {"]
#[doc = "      \"description\": \"HTTP headers to send with requests to the MCP server\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the MCP server\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"URL of the MCP server\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct McpServer {
    #[doc = "HTTP headers to send with requests to the MCP server"]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "Name of the MCP server"]
    pub name: ::std::string::String,
    #[doc = "URL of the MCP server"]
    pub url: ::std::string::String,
}
impl McpServer {
    pub fn builder() -> builder::McpServer {
        Default::default()
    }
}
#[doc = "`Mount`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"host\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"host\": {"]
#[doc = "      \"description\": \"Path in the host filesystem. Can use $SOURCES or $HOME variables (e.g., \\\"$SOURCES/../main\\\", \\\"$HOME/.gitconfig\\\", \\\"/absolute/path\\\")\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ro\": {"]
#[doc = "      \"description\": \"Mount as read-only. Defaults to false (read-write).\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"description\": \"Path in the workspace filesystem where the host path will be mounted. Can use $SOURCES or $HOME variables.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Mount {
    #[doc = "Path in the host filesystem. Can use $SOURCES or $HOME variables (e.g., \"$SOURCES/../main\", \"$HOME/.gitconfig\", \"/absolute/path\")"]
    pub host: ::std::string::String,
    #[doc = "Mount as read-only. Defaults to false (read-write)."]
    #[serde(default)]
    pub ro: bool,
    #[doc = "Path in the workspace filesystem where the host path will be mounted. Can use $SOURCES or $HOME variables."]
    pub target: ::std::string::String,
}
impl Mount {
    pub fn builder() -> builder::Mount {
        Default::default()
    }
}
#[doc = "`NetworkConfiguration`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"hosts\": {"]
#[doc = "      \"description\": \"List of hostnames to allow in deny mode.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mode\": {"]
#[doc = "      \"description\": \"Network access mode. 'allow' permits all. 'deny' blocks everything except the specified hosts/CIDRs. Defaults to 'deny'.\","]
#[doc = "      \"default\": \"deny\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"allow\","]
#[doc = "        \"deny\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetworkConfiguration {
    #[doc = "List of hostnames to allow in deny mode."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hosts: ::std::vec::Vec<::std::string::String>,
    #[doc = "Network access mode. 'allow' permits all. 'deny' blocks everything except the specified hosts/CIDRs. Defaults to 'deny'."]
    #[serde(default = "defaults::network_configuration_mode")]
    pub mode: NetworkConfigurationMode,
}
impl ::std::default::Default for NetworkConfiguration {
    fn default() -> Self {
        Self {
            hosts: Default::default(),
            mode: defaults::network_configuration_mode(),
        }
    }
}
impl NetworkConfiguration {
    pub fn builder() -> builder::NetworkConfiguration {
        Default::default()
    }
}
#[doc = "Network access mode. 'allow' permits all. 'deny' blocks everything except the specified hosts/CIDRs. Defaults to 'deny'."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Network access mode. 'allow' permits all. 'deny' blocks everything except the specified hosts/CIDRs. Defaults to 'deny'.\","]
#[doc = "  \"default\": \"deny\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"allow\","]
#[doc = "    \"deny\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkConfigurationMode {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}
impl ::std::fmt::Display for NetworkConfigurationMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}
impl ::std::str::FromStr for NetworkConfigurationMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkConfigurationMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkConfigurationMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkConfigurationMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for NetworkConfigurationMode {
    fn default() -> Self {
        NetworkConfigurationMode::Deny
    }
}
#[doc = "`WorkspaceConfiguration`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"environment\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/EnvironmentVariable\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"features\": {"]
#[doc = "      \"description\": \"Features to add to the workspace, compatible with devcontainers features (https://containers.dev/implementors/features/). Keys are feature identifiers (e.g. OCI image references), values are feature-specific options.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mcp\": {"]
#[doc = "      \"$ref\": \"#/definitions/McpConfiguration\""]
#[doc = "    },"]
#[doc = "    \"mounts\": {"]
#[doc = "      \"description\": \"List of directories to mount in the workspace. Supports $SOURCES (workspace sources directory) and $HOME (user home directory) variables.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Mount\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"network\": {"]
#[doc = "      \"$ref\": \"#/definitions/NetworkConfiguration\""]
#[doc = "    },"]
#[doc = "    \"ports\": {"]
#[doc = "      \"description\": \"List of TCP ports to expose from the workspace.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"maximum\": 65535.0,"]
#[doc = "        \"minimum\": 1.0"]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"secrets\": {"]
#[doc = "      \"description\": \"List of secret names to inject into the workspace.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"skills\": {"]
#[doc = "      \"description\": \"List of folders containing skills (SKILL.md and related files) to be provided to the agent.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceConfiguration {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub environment: ::std::vec::Vec<EnvironmentVariable>,
    #[doc = "Features to add to the workspace, compatible with devcontainers features (https://containers.dev/implementors/features/). Keys are feature identifiers (e.g. OCI image references), values are feature-specific options."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub features: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mcp: ::std::option::Option<McpConfiguration>,
    #[doc = "List of directories to mount in the workspace. Supports $SOURCES (workspace sources directory) and $HOME (user home directory) variables."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mounts: ::std::vec::Vec<Mount>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub network: ::std::option::Option<NetworkConfiguration>,
    #[doc = "List of TCP ports to expose from the workspace."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ports: ::std::option::Option<Vec<::std::num::NonZeroU64>>,
    #[doc = "List of secret names to inject into the workspace."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub secrets: ::std::vec::Vec<::std::string::String>,
    #[doc = "List of folders containing skills (SKILL.md and related files) to be provided to the agent."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for WorkspaceConfiguration {
    fn default() -> Self {
        Self {
            environment: Default::default(),
            features: Default::default(),
            mcp: Default::default(),
            mounts: Default::default(),
            network: Default::default(),
            ports: Default::default(),
            secrets: Default::default(),
            skills: Default::default(),
        }
    }
}
impl WorkspaceConfiguration {
    pub fn builder() -> builder::WorkspaceConfiguration {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct EnvironmentVariable {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        secret: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EnvironmentVariable {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                secret: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl EnvironmentVariable {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secret: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EnvironmentVariable> for super::EnvironmentVariable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnvironmentVariable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                secret: value.secret?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::EnvironmentVariable> for EnvironmentVariable {
        fn from(value: super::EnvironmentVariable) -> Self {
            Self {
                name: Ok(value.name),
                secret: Ok(value.secret),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpCommand {
        args: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        command: ::std::result::Result<::std::string::String, ::std::string::String>,
        env: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for McpCommand {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                command: Err("no value supplied for command".to_string()),
                env: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl McpCommand {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {e}"));
            self
        }
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpCommand> for super::McpCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                command: value.command?,
                env: value.env?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::McpCommand> for McpCommand {
        fn from(value: super::McpCommand) -> Self {
            Self {
                args: Ok(value.args),
                command: Ok(value.command),
                env: Ok(value.env),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpConfiguration {
        commands: ::std::result::Result<::std::vec::Vec<super::McpCommand>, ::std::string::String>,
        servers: ::std::result::Result<::std::vec::Vec<super::McpServer>, ::std::string::String>,
    }
    impl ::std::default::Default for McpConfiguration {
        fn default() -> Self {
            Self {
                commands: Ok(Default::default()),
                servers: Ok(Default::default()),
            }
        }
    }
    impl McpConfiguration {
        pub fn commands<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::McpCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.commands = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for commands: {e}"));
            self
        }
        pub fn servers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::McpServer>>,
            T::Error: ::std::fmt::Display,
        {
            self.servers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for servers: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpConfiguration> for super::McpConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                commands: value.commands?,
                servers: value.servers?,
            })
        }
    }
    impl ::std::convert::From<super::McpConfiguration> for McpConfiguration {
        fn from(value: super::McpConfiguration) -> Self {
            Self {
                commands: Ok(value.commands),
                servers: Ok(value.servers),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpServer {
        headers: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for McpServer {
        fn default() -> Self {
            Self {
                headers: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl McpServer {
        pub fn headers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.headers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for headers: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpServer> for super::McpServer {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpServer,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                headers: value.headers?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::McpServer> for McpServer {
        fn from(value: super::McpServer) -> Self {
            Self {
                headers: Ok(value.headers),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mount {
        host: ::std::result::Result<::std::string::String, ::std::string::String>,
        ro: ::std::result::Result<bool, ::std::string::String>,
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Mount {
        fn default() -> Self {
            Self {
                host: Err("no value supplied for host".to_string()),
                ro: Ok(Default::default()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl Mount {
        pub fn host<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.host = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host: {e}"));
            self
        }
        pub fn ro<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.ro = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ro: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Mount> for super::Mount {
        type Error = super::error::ConversionError;
        fn try_from(value: Mount) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                host: value.host?,
                ro: value.ro?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::Mount> for Mount {
        fn from(value: super::Mount) -> Self {
            Self {
                host: Ok(value.host),
                ro: Ok(value.ro),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NetworkConfiguration {
        hosts: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        mode: ::std::result::Result<super::NetworkConfigurationMode, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkConfiguration {
        fn default() -> Self {
            Self {
                hosts: Ok(Default::default()),
                mode: Ok(super::defaults::network_configuration_mode()),
            }
        }
    }
    impl NetworkConfiguration {
        pub fn hosts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hosts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hosts: {e}"));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NetworkConfigurationMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NetworkConfiguration> for super::NetworkConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                hosts: value.hosts?,
                mode: value.mode?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkConfiguration> for NetworkConfiguration {
        fn from(value: super::NetworkConfiguration) -> Self {
            Self {
                hosts: Ok(value.hosts),
                mode: Ok(value.mode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceConfiguration {
        environment: ::std::result::Result<
            ::std::vec::Vec<super::EnvironmentVariable>,
            ::std::string::String,
        >,
        features: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            ::std::string::String,
        >,
        mcp: ::std::result::Result<
            ::std::option::Option<super::McpConfiguration>,
            ::std::string::String,
        >,
        mounts: ::std::result::Result<::std::vec::Vec<super::Mount>, ::std::string::String>,
        network: ::std::result::Result<
            ::std::option::Option<super::NetworkConfiguration>,
            ::std::string::String,
        >,
        ports: ::std::result::Result<
            ::std::option::Option<Vec<::std::num::NonZeroU64>>,
            ::std::string::String,
        >,
        secrets:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        skills:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceConfiguration {
        fn default() -> Self {
            Self {
                environment: Ok(Default::default()),
                features: Ok(Default::default()),
                mcp: Ok(Default::default()),
                mounts: Ok(Default::default()),
                network: Ok(Default::default()),
                ports: Ok(Default::default()),
                secrets: Ok(Default::default()),
                skills: Ok(Default::default()),
            }
        }
    }
    impl WorkspaceConfiguration {
        pub fn environment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EnvironmentVariable>>,
            T::Error: ::std::fmt::Display,
        {
            self.environment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for environment: {e}"));
            self
        }
        pub fn features<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.features = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for features: {e}"));
            self
        }
        pub fn mcp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::McpConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp: {e}"));
            self
        }
        pub fn mounts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Mount>>,
            T::Error: ::std::fmt::Display,
        {
            self.mounts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mounts: {e}"));
            self
        }
        pub fn network<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.network = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network: {e}"));
            self
        }
        pub fn ports<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::num::NonZeroU64>>>,
            T::Error: ::std::fmt::Display,
        {
            self.ports = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ports: {e}"));
            self
        }
        pub fn secrets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.secrets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secrets: {e}"));
            self
        }
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceConfiguration> for super::WorkspaceConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                environment: value.environment?,
                features: value.features?,
                mcp: value.mcp?,
                mounts: value.mounts?,
                network: value.network?,
                ports: value.ports?,
                secrets: value.secrets?,
                skills: value.skills?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceConfiguration> for WorkspaceConfiguration {
        fn from(value: super::WorkspaceConfiguration) -> Self {
            Self {
                environment: Ok(value.environment),
                features: Ok(value.features),
                mcp: Ok(value.mcp),
                mounts: Ok(value.mounts),
                network: Ok(value.network),
                ports: Ok(value.ports),
                secrets: Ok(value.secrets),
                skills: Ok(value.skills),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn network_configuration_mode() -> super::NetworkConfigurationMode {
        super::NetworkConfigurationMode::Deny
    }
}
