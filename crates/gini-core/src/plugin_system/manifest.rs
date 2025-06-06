use crate::plugin_system::version::VersionRange;
use crate::plugin_system::traits::PluginPriority;
use serde::{Deserialize, Serialize}; // Added Serialize
use crate::plugin_system::dependency::PluginDependency; // Import PluginDependency
use std::path::PathBuf; // Added for plugin_base_dir

/// Defines the type of access a plugin requires or provides for a resource.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)] // Added Serialize
pub enum ResourceAccessType {
    /// Claims exclusive control/write access. No other plugin can claim Exclusive or SharedWrite.
    #[serde(rename = "exclusive_write")]
    ExclusiveWrite,

    /// Claims shared read access. Compatible with other SharedRead claims.
    #[serde(rename = "shared_read")]
    SharedRead,

    /// Indicates the plugin provides a unique entity identified by `identifier` of `resource_type`.
    /// e.g., A plugin `provides_unique_id` for a `stage_id`. Conflicts if another plugin also `provides_unique_id` for the same stage_id.
    #[serde(rename = "provides_unique_id")]
    ProvidesUniqueId,
}

impl ResourceAccessType {
    /// Returns a string representation of the access type.
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceAccessType::ExclusiveWrite => "ExclusiveWrite",
            ResourceAccessType::SharedRead => "SharedRead",
            ResourceAccessType::ProvidesUniqueId => "ProvidesUniqueId",
        }
    }
}

/// Describes a resource a plugin claims or provides.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)] // Added Serialize
pub struct ResourceClaim {
    /// Type of the resource (e.g., "file_path", "network_port", "stage_id", "config_scope").
    /// Standardized strings should be used.
    #[serde(rename = "type")]
    pub resource_type: String,

    /// Unique identifier for the resource within its type.
    /// For "file_path": "/var/log/app.log"
    /// For "network_port": "8080"
    /// For "stage_id": "data_processing_stage"
    /// For "config_scope": "global.settings.network"
    pub identifier: String,

    /// Describes how the plugin interacts with the resource.
    pub access: ResourceAccessType,
}

/// Represents a plugin manifest that describes a plugin
#[derive(Debug, Clone, Serialize)] // Added Serialize, Removed Deserialize
pub struct PluginManifest {
    /// Unique identifier for the plugin
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin description
    pub description: String,

    /// Plugin author
    pub author: String,

    /// Plugin website URL (optional)
    pub website: Option<String>, // Removed serde attribute

    /// License information
    pub license: Option<String>, // Removed serde attribute

    /// Compatible API versions
    pub api_versions: Vec<VersionRange>, // Changed back to Vec<VersionRange>

    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>, // Use PluginDependency

    /// Whether this is a core plugin
    pub is_core: bool, // Removed serde attribute

    /// Plugin priority (String representation)
    pub priority: Option<String>, // Removed serde attribute

    /// Entry point (library name or script path)
    pub entry_point: String, // Changed back to String

    /// Additional plugin files
    pub files: Vec<String>, // Removed serde attribute

    /// Plugin configuration schema (optional)
    pub config_schema: Option<String>, // Removed serde attribute

    /// Tags for categorization
    pub tags: Vec<String>, // Removed serde attribute

    /// List of plugin IDs this plugin conflicts with (cannot run together)
    pub conflicts_with: Vec<String>, // Added for conflict detection

    /// List of plugins/versions this plugin is incompatible with
    pub incompatible_with: Vec<PluginDependency>, // Use PluginDependency

    /// Resource claims made by the plugin.
    pub resources: Vec<ResourceClaim>,

    /// The base directory of the plugin, where its manifest.json is located.
    /// This is set by the PluginLoader during manifest scanning.
    #[serde(with = "pathbuf_serde_helper")]
    pub plugin_base_dir: PathBuf,
}

mod pathbuf_serde_helper {
    use std::path::PathBuf;
    use serde::{self, Serializer}; // Removed Deserializer, Deserialize

    pub fn serialize<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(path.to_str().unwrap_or(""))
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     String::deserialize(deserializer).map(PathBuf::from)
    // }
}

// Removed DependencyInfo struct definition

impl PluginManifest {
    /// Create a new plugin manifest
    pub fn new(id: &str, name: &str, version: &str, description: &str, author: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            website: None,
            license: None,
            api_versions: Vec::new(), // Expects VersionRange now
            dependencies: Vec::new(), // Expects PluginDependency
            is_core: false,
            priority: None,
            entry_point: format!("lib{}.so", id), // Back to String
            files: Vec::new(),
            config_schema: None,
            tags: Vec::new(),
            conflicts_with: Vec::new(), // Initialize new field
            incompatible_with: Vec::new(), // Initialize new field (Vec<PluginDependency>)
            resources: Vec::new(),
            plugin_base_dir: PathBuf::new(), // Initialize with an empty path
        }
    }

    /// Add an API version range
    pub fn add_api_version(&mut self, version_range: VersionRange) -> &mut Self { // Changed param type
        self.api_versions.push(version_range);
        self
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, id: &str, version_range: Option<VersionRange>, required: bool) -> &mut Self { // Changed param type
        self.dependencies.push(PluginDependency { // Use PluginDependency struct literal
            plugin_name: id.to_string(), // Field name is plugin_name
            version_range,
            required,
        });
        self
    }

    /// Set the plugin priority
    pub fn set_priority(&mut self, priority: PluginPriority) -> &mut Self {
        self.priority = Some(priority.to_string());
        self
    }

    /// Mark this as a core plugin
    pub fn set_core(&mut self, is_core: bool) -> &mut Self {
        self.is_core = is_core;
        self
    }

    /// Add a tag to the plugin
    pub fn add_tag(&mut self, tag: &str) -> &mut Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Add a plugin ID that this plugin conflicts with
    pub fn add_conflict(&mut self, id: &str) -> &mut Self {
        self.conflicts_with.push(id.to_string());
        self
    }

    /// Add an incompatibility rule
    pub fn add_incompatibility(&mut self, id: &str, version_range: Option<VersionRange>) -> &mut Self {
        // Incompatibility implies it's required for the check to matter, but the 'required' field
        // in PluginDependency isn't strictly necessary here, maybe default to true or ignore?
        // Let's store it like a dependency for structure reuse.
        self.incompatible_with.push(PluginDependency { // Use PluginDependency struct literal
            plugin_name: id.to_string(), // Field name is plugin_name
            version_range,
            required: true, // Mark as true conceptually for the check (or maybe false?)
        });
        self
    }

    /// Get the plugin priority
    pub fn get_priority(&self) -> Option<PluginPriority> {
        self.priority.as_ref().and_then(|p| PluginPriority::from_str(p)) // from_str already returns Option
    }

    // Removed get_entry_point as entry_point is String again
    // Removed get_api_versions as api_versions is Vec<VersionRange> again
    // Removed get_dependencies as dependencies is Vec<PluginDependency> again
}

/// Builder for creating a plugin manifest
pub struct ManifestBuilder {
    manifest: PluginManifest,
}

impl ManifestBuilder {
    /// Create a new manifest builder
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        Self {
            manifest: PluginManifest::new(
                id,
                name,
                version,
                "A plugin for OSX-Forge", // Default description
                "Unknown", // Default author
            ),
        }
    }

    /// Set the plugin description
    pub fn description(mut self, description: &str) -> Self {
        self.manifest.description = description.to_string();
        self
    }

    /// Set the plugin author
    pub fn author(mut self, author: &str) -> Self {
        self.manifest.author = author.to_string();
        self
    }

    /// Set the plugin website
    pub fn website(mut self, website: &str) -> Self {
        self.manifest.website = Some(website.to_string());
        self
    }

    /// Set the plugin license
    pub fn license(mut self, license: &str) -> Self {
        self.manifest.license = Some(license.to_string());
        self
    }

    /// Add an API version compatibility range
    pub fn api_version(mut self, version_range: VersionRange) -> Self { // Changed param type
        self.manifest.add_api_version(version_range);
        self
    }

    /// Add a dependency
    pub fn dependency(mut self, id: &str, version_range: Option<VersionRange>, required: bool) -> Self { // Changed param type
        self.manifest.add_dependency(id, version_range, required);
        self
    }

    /// Add a conflict rule (plugin ID this one conflicts with)
    pub fn conflict(mut self, id: &str) -> Self {
        self.manifest.add_conflict(id);
        self
    }

    /// Add an incompatibility rule (plugin ID and optional version range)
    pub fn incompatibility(mut self, id: &str, version_range: Option<VersionRange>) -> Self {
        self.manifest.add_incompatibility(id, version_range);
        self
    }

    /// Set whether this is a core plugin
    pub fn core(mut self, is_core: bool) -> Self {
        self.manifest.is_core = is_core;
        self
    }

    /// Set the plugin priority
    pub fn priority(mut self, priority: PluginPriority) -> Self {
        self.manifest.set_priority(priority);
        self
    }

    /// Set the entry point
    pub fn entry_point(mut self, entry_point: &str) -> Self { // Changed param type
        self.manifest.entry_point = entry_point.to_string(); // Assign directly to String
        self
    }

    /// Add a file to the plugin
    pub fn file(mut self, file: &str) -> Self {
        self.manifest.files.push(file.to_string());
        self
    }

    /// Add multiple files to the plugin
    pub fn files(mut self, files: &[&str]) -> Self {
        for file in files {
            self.manifest.files.push(file.to_string());
        }
        self
    }

    /// Add a tag to the plugin
    pub fn tag(mut self, tag: &str) -> Self {
        self.manifest.add_tag(tag);
        self
    }

    /// Add multiple tags to the plugin
    pub fn tags(mut self, tags: &[&str]) -> Self {
        for tag in tags {
            self.manifest.add_tag(tag);
        }
        self
    }

    /// Adds a resource claim to the manifest.
    ///
    /// # Arguments
    ///
    /// * `resource_type`: Type of the resource (e.g., "file_path", "network_port").
    /// * `identifier`: Unique identifier for the resource (e.g., "/path/to/file", "8080").
    /// * `access`: The type of access required for the resource.
    pub fn resource(mut self, resource_type: &str, identifier: &str, access: ResourceAccessType) -> Self {
        self.manifest.resources.push(ResourceClaim {
            resource_type: resource_type.to_string(),
            identifier: identifier.to_string(),
            access,
        });
        self
    }

    /// Build the manifest
    pub fn build(self) -> PluginManifest {
        self.manifest
    }
}