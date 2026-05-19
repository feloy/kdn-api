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
#[doc = "`Error`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub error: ::std::string::String,
}
impl Error {
    pub fn builder() -> builder::Error {
        Default::default()
    }
}
#[doc = "`Info`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agents\","]
#[doc = "    \"runtimes\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"runtimes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Info {
    pub agents: ::std::vec::Vec<::std::string::String>,
    pub runtimes: ::std::vec::Vec<::std::string::String>,
    pub version: ::std::string::String,
}
impl Info {
    pub fn builder() -> builder::Info {
        Default::default()
    }
}
#[doc = "`ProviderInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"params\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ProviderParam\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProviderInfo {
    pub name: ::std::string::String,
    pub params: ::std::vec::Vec<ProviderParam>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ProviderInfo {
    pub fn builder() -> builder::ProviderInfo {
        Default::default()
    }
}
#[doc = "`ProviderName`"]
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
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProviderName {
    pub name: ::std::string::String,
}
impl ProviderName {
    pub fn builder() -> builder::ProviderName {
        Default::default()
    }
}
#[doc = "`ProviderParam`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProviderParam {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}
impl ProviderParam {
    pub fn builder() -> builder::ProviderParam {
        Default::default()
    }
}
#[doc = "`ProvidersList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ProviderInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProvidersList {
    pub items: ::std::vec::Vec<ProviderInfo>,
}
impl ProvidersList {
    pub fn builder() -> builder::ProvidersList {
        Default::default()
    }
}
#[doc = "`RuntimeInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"local\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"local\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInfo {
    pub description: ::std::string::String,
    pub local: bool,
    pub name: ::std::string::String,
}
impl RuntimeInfo {
    pub fn builder() -> builder::RuntimeInfo {
        Default::default()
    }
}
#[doc = "`RuntimesList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/RuntimeInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimesList {
    pub items: ::std::vec::Vec<RuntimeInfo>,
}
impl RuntimesList {
    pub fn builder() -> builder::RuntimesList {
        Default::default()
    }
}
#[doc = "`SecretInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"envs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"header\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"headerTemplate\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hosts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecretInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub envs: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "headerTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub header_template: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hosts: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl SecretInfo {
    pub fn builder() -> builder::SecretInfo {
        Default::default()
    }
}
#[doc = "`SecretName`"]
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
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecretName {
    pub name: ::std::string::String,
}
impl SecretName {
    pub fn builder() -> builder::SecretName {
        Default::default()
    }
}
#[doc = "`SecretService`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"headerName\","]
#[doc = "    \"hostPattern\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"envVars\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headerName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"headerTemplate\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hostPattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecretService {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "envVars",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub env_vars: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "headerName")]
    pub header_name: ::std::string::String,
    #[serde(
        rename = "headerTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub header_template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostPattern")]
    pub host_pattern: ::std::string::String,
    pub name: ::std::string::String,
}
impl SecretService {
    pub fn builder() -> builder::SecretService {
        Default::default()
    }
}
#[doc = "`SecretServicesList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SecretService\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecretServicesList {
    pub items: ::std::vec::Vec<SecretService>,
}
impl SecretServicesList {
    pub fn builder() -> builder::SecretServicesList {
        Default::default()
    }
}
#[doc = "`SecretsList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SecretInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecretsList {
    pub items: ::std::vec::Vec<SecretInfo>,
}
impl SecretsList {
    pub fn builder() -> builder::SecretsList {
        Default::default()
    }
}
#[doc = "`Workspace`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"forwards\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"paths\","]
#[doc = "    \"project\","]
#[doc = "    \"runtime\","]
#[doc = "    \"state\","]
#[doc = "    \"timestamps\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"forwards\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/WorkspaceForward\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        {"]
#[doc = "          \"bind\": \"0.0.0.0\","]
#[doc = "          \"port\": 18080,"]
#[doc = "          \"target\": 8080"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"bind\": \"localhost\","]
#[doc = "          \"port\": 5433,"]
#[doc = "          \"target\": 5432"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"paths\": {"]
#[doc = "      \"$ref\": \"#/definitions/WorkspacePaths\""]
#[doc = "    },"]
#[doc = "    \"project\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"runtime\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"$ref\": \"#/definitions/WorkspaceState\""]
#[doc = "    },"]
#[doc = "    \"timestamps\": {"]
#[doc = "      \"$ref\": \"#/definitions/WorkspaceTimestamps\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    pub agent: ::std::string::String,
    pub forwards: ::std::vec::Vec<WorkspaceForward>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub paths: WorkspacePaths,
    pub project: ::std::string::String,
    pub runtime: ::std::string::String,
    pub state: WorkspaceState,
    pub timestamps: WorkspaceTimestamps,
}
impl Workspace {
    pub fn builder() -> builder::Workspace {
        Default::default()
    }
}
#[doc = "`WorkspaceForward`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bind\","]
#[doc = "    \"port\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bind\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceForward {
    pub bind: ::std::string::String,
    pub port: i64,
    pub target: i64,
}
impl WorkspaceForward {
    pub fn builder() -> builder::WorkspaceForward {
        Default::default()
    }
}
#[doc = "`WorkspaceId`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceId {
    pub id: ::std::string::String,
}
impl WorkspaceId {
    pub fn builder() -> builder::WorkspaceId {
        Default::default()
    }
}
#[doc = "`WorkspacePaths`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configuration\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configuration\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspacePaths {
    pub configuration: ::std::string::String,
    pub source: ::std::string::String,
}
impl WorkspacePaths {
    pub fn builder() -> builder::WorkspacePaths {
        Default::default()
    }
}
#[doc = "`WorkspaceState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"stopped\","]
#[doc = "    \"running\","]
#[doc = "    \"error\","]
#[doc = "    \"unknown\""]
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
pub enum WorkspaceState {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for WorkspaceState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stopped => f.write_str("stopped"),
            Self::Running => f.write_str("running"),
            Self::Error => f.write_str("error"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for WorkspaceState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stopped" => Ok(Self::Stopped),
            "running" => Ok(Self::Running),
            "error" => Ok(Self::Error),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WorkspaceState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WorkspaceState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkspaceState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WorkspaceTimestamps`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"created\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"started\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceTimestamps {
    pub created: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub started: ::std::option::Option<i64>,
}
impl WorkspaceTimestamps {
    pub fn builder() -> builder::WorkspaceTimestamps {
        Default::default()
    }
}
#[doc = "`WorkspacesList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Workspace\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspacesList {
    pub items: ::std::vec::Vec<Workspace>,
}
impl WorkspacesList {
    pub fn builder() -> builder::WorkspacesList {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Error {
        error: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Error {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
            }
        }
    }
    impl Error {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Error> for super::Error {
        type Error = super::error::ConversionError;
        fn try_from(value: Error) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
            })
        }
    }
    impl ::std::convert::From<super::Error> for Error {
        fn from(value: super::Error) -> Self {
            Self {
                error: Ok(value.error),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Info {
        agents:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        runtimes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Info {
        fn default() -> Self {
            Self {
                agents: Err("no value supplied for agents".to_string()),
                runtimes: Err("no value supplied for runtimes".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Info {
        pub fn agents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agents: {e}"));
            self
        }
        pub fn runtimes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.runtimes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for runtimes: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Info> for super::Info {
        type Error = super::error::ConversionError;
        fn try_from(value: Info) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agents: value.agents?,
                runtimes: value.runtimes?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Info> for Info {
        fn from(value: super::Info) -> Self {
            Self {
                agents: Ok(value.agents),
                runtimes: Ok(value.runtimes),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProviderInfo {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<::std::vec::Vec<super::ProviderParam>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProviderInfo {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                params: Err("no value supplied for params".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ProviderInfo {
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
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProviderParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProviderInfo> for super::ProviderInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProviderInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                params: value.params?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ProviderInfo> for ProviderInfo {
        fn from(value: super::ProviderInfo) -> Self {
            Self {
                name: Ok(value.name),
                params: Ok(value.params),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProviderName {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProviderName {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ProviderName {
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
    impl ::std::convert::TryFrom<ProviderName> for super::ProviderName {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProviderName,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ProviderName> for ProviderName {
        fn from(value: super::ProviderName) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProviderParam {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProviderParam {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ProviderParam {
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
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProviderParam> for super::ProviderParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProviderParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ProviderParam> for ProviderParam {
        fn from(value: super::ProviderParam) -> Self {
            Self {
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProvidersList {
        items: ::std::result::Result<::std::vec::Vec<super::ProviderInfo>, ::std::string::String>,
    }
    impl ::std::default::Default for ProvidersList {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl ProvidersList {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProviderInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProvidersList> for super::ProvidersList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProvidersList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::ProvidersList> for ProvidersList {
        fn from(value: super::ProvidersList) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RuntimeInfo {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        local: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RuntimeInfo {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                local: Err("no value supplied for local".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl RuntimeInfo {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn local<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.local = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for local: {e}"));
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
    impl ::std::convert::TryFrom<RuntimeInfo> for super::RuntimeInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RuntimeInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                local: value.local?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::RuntimeInfo> for RuntimeInfo {
        fn from(value: super::RuntimeInfo) -> Self {
            Self {
                description: Ok(value.description),
                local: Ok(value.local),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RuntimesList {
        items: ::std::result::Result<::std::vec::Vec<super::RuntimeInfo>, ::std::string::String>,
    }
    impl ::std::default::Default for RuntimesList {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl RuntimesList {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RuntimeInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RuntimesList> for super::RuntimesList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RuntimesList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::RuntimesList> for RuntimesList {
        fn from(value: super::RuntimesList) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecretInfo {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        envs: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        header: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        header_template: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hosts: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SecretInfo {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                envs: Ok(Default::default()),
                header: Ok(Default::default()),
                header_template: Ok(Default::default()),
                hosts: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                path: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SecretInfo {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn envs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.envs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for envs: {e}"));
            self
        }
        pub fn header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.header = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header: {e}"));
            self
        }
        pub fn header_template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.header_template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header_template: {e}"));
            self
        }
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
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SecretInfo> for super::SecretInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecretInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                envs: value.envs?,
                header: value.header?,
                header_template: value.header_template?,
                hosts: value.hosts?,
                name: value.name?,
                path: value.path?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SecretInfo> for SecretInfo {
        fn from(value: super::SecretInfo) -> Self {
            Self {
                description: Ok(value.description),
                envs: Ok(value.envs),
                header: Ok(value.header),
                header_template: Ok(value.header_template),
                hosts: Ok(value.hosts),
                name: Ok(value.name),
                path: Ok(value.path),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecretName {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SecretName {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl SecretName {
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
    impl ::std::convert::TryFrom<SecretName> for super::SecretName {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecretName,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::SecretName> for SecretName {
        fn from(value: super::SecretName) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecretService {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        env_vars:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        header_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        header_template: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        host_pattern: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SecretService {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                env_vars: Ok(Default::default()),
                header_name: Err("no value supplied for header_name".to_string()),
                header_template: Ok(Default::default()),
                host_pattern: Err("no value supplied for host_pattern".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl SecretService {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn env_vars<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.env_vars = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env_vars: {e}"));
            self
        }
        pub fn header_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.header_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header_name: {e}"));
            self
        }
        pub fn header_template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.header_template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header_template: {e}"));
            self
        }
        pub fn host_pattern<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.host_pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host_pattern: {e}"));
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
    impl ::std::convert::TryFrom<SecretService> for super::SecretService {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecretService,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                env_vars: value.env_vars?,
                header_name: value.header_name?,
                header_template: value.header_template?,
                host_pattern: value.host_pattern?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::SecretService> for SecretService {
        fn from(value: super::SecretService) -> Self {
            Self {
                description: Ok(value.description),
                env_vars: Ok(value.env_vars),
                header_name: Ok(value.header_name),
                header_template: Ok(value.header_template),
                host_pattern: Ok(value.host_pattern),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecretServicesList {
        items: ::std::result::Result<::std::vec::Vec<super::SecretService>, ::std::string::String>,
    }
    impl ::std::default::Default for SecretServicesList {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl SecretServicesList {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SecretService>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SecretServicesList> for super::SecretServicesList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecretServicesList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::SecretServicesList> for SecretServicesList {
        fn from(value: super::SecretServicesList) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecretsList {
        items: ::std::result::Result<::std::vec::Vec<super::SecretInfo>, ::std::string::String>,
    }
    impl ::std::default::Default for SecretsList {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl SecretsList {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SecretInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SecretsList> for super::SecretsList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecretsList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::SecretsList> for SecretsList {
        fn from(value: super::SecretsList) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Workspace {
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        forwards:
            ::std::result::Result<::std::vec::Vec<super::WorkspaceForward>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        paths: ::std::result::Result<super::WorkspacePaths, ::std::string::String>,
        project: ::std::result::Result<::std::string::String, ::std::string::String>,
        runtime: ::std::result::Result<::std::string::String, ::std::string::String>,
        state: ::std::result::Result<super::WorkspaceState, ::std::string::String>,
        timestamps: ::std::result::Result<super::WorkspaceTimestamps, ::std::string::String>,
    }
    impl ::std::default::Default for Workspace {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                forwards: Err("no value supplied for forwards".to_string()),
                id: Err("no value supplied for id".to_string()),
                model: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                paths: Err("no value supplied for paths".to_string()),
                project: Err("no value supplied for project".to_string()),
                runtime: Err("no value supplied for runtime".to_string()),
                state: Err("no value supplied for state".to_string()),
                timestamps: Err("no value supplied for timestamps".to_string()),
            }
        }
    }
    impl Workspace {
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn forwards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WorkspaceForward>>,
            T::Error: ::std::fmt::Display,
        {
            self.forwards = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for forwards: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
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
        pub fn paths<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WorkspacePaths>,
            T::Error: ::std::fmt::Display,
        {
            self.paths = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paths: {e}"));
            self
        }
        pub fn project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project: {e}"));
            self
        }
        pub fn runtime<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.runtime = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for runtime: {e}"));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WorkspaceState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
        pub fn timestamps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WorkspaceTimestamps>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamps: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Workspace> for super::Workspace {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Workspace,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                forwards: value.forwards?,
                id: value.id?,
                model: value.model?,
                name: value.name?,
                paths: value.paths?,
                project: value.project?,
                runtime: value.runtime?,
                state: value.state?,
                timestamps: value.timestamps?,
            })
        }
    }
    impl ::std::convert::From<super::Workspace> for Workspace {
        fn from(value: super::Workspace) -> Self {
            Self {
                agent: Ok(value.agent),
                forwards: Ok(value.forwards),
                id: Ok(value.id),
                model: Ok(value.model),
                name: Ok(value.name),
                paths: Ok(value.paths),
                project: Ok(value.project),
                runtime: Ok(value.runtime),
                state: Ok(value.state),
                timestamps: Ok(value.timestamps),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceForward {
        bind: ::std::result::Result<::std::string::String, ::std::string::String>,
        port: ::std::result::Result<i64, ::std::string::String>,
        target: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceForward {
        fn default() -> Self {
            Self {
                bind: Err("no value supplied for bind".to_string()),
                port: Err("no value supplied for port".to_string()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl WorkspaceForward {
        pub fn bind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bind: {e}"));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceForward> for super::WorkspaceForward {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceForward,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bind: value.bind?,
                port: value.port?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceForward> for WorkspaceForward {
        fn from(value: super::WorkspaceForward) -> Self {
            Self {
                bind: Ok(value.bind),
                port: Ok(value.port),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceId {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceId {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl WorkspaceId {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceId> for super::WorkspaceId {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceId,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::WorkspaceId> for WorkspaceId {
        fn from(value: super::WorkspaceId) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspacePaths {
        configuration: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspacePaths {
        fn default() -> Self {
            Self {
                configuration: Err("no value supplied for configuration".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl WorkspacePaths {
        pub fn configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.configuration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configuration: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspacePaths> for super::WorkspacePaths {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspacePaths,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration: value.configuration?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspacePaths> for WorkspacePaths {
        fn from(value: super::WorkspacePaths) -> Self {
            Self {
                configuration: Ok(value.configuration),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceTimestamps {
        created: ::std::result::Result<i64, ::std::string::String>,
        started: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceTimestamps {
        fn default() -> Self {
            Self {
                created: Err("no value supplied for created".to_string()),
                started: Ok(Default::default()),
            }
        }
    }
    impl WorkspaceTimestamps {
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn started<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.started = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceTimestamps> for super::WorkspaceTimestamps {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceTimestamps,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created: value.created?,
                started: value.started?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceTimestamps> for WorkspaceTimestamps {
        fn from(value: super::WorkspaceTimestamps) -> Self {
            Self {
                created: Ok(value.created),
                started: Ok(value.started),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspacesList {
        items: ::std::result::Result<::std::vec::Vec<super::Workspace>, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspacesList {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl WorkspacesList {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Workspace>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspacesList> for super::WorkspacesList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspacesList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspacesList> for WorkspacesList {
        fn from(value: super::WorkspacesList) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
}
