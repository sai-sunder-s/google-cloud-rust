// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Parameter Manager API.
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// `ParameterManager` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ParameterManager` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ParameterManager` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ParameterManager {
    inner: Arc<dyn super::stub::dynamic::ParameterManager>,
}

impl ParameterManager {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::ParameterManager + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ParameterManager>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ParameterManager> {
        super::transport::ParameterManager::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ParameterManager> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ParameterManager::new)
    }

    /// Lists Parameters in a given project and location.
    pub fn list_parameters(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::ListParameters {
        super::builder::parameter_manager::ListParameters::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Parameter.
    pub fn get_parameter(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::GetParameter {
        super::builder::parameter_manager::GetParameter::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new Parameter in a given project and location.
    pub fn create_parameter(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::CreateParameter {
        super::builder::parameter_manager::CreateParameter::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a single Parameter.
    pub fn update_parameter(
        &self,
        parameter: impl Into<crate::model::Parameter>,
    ) -> super::builder::parameter_manager::UpdateParameter {
        super::builder::parameter_manager::UpdateParameter::new(self.inner.clone())
            .set_parameter(parameter.into())
    }

    /// Deletes a single Parameter.
    pub fn delete_parameter(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::DeleteParameter {
        super::builder::parameter_manager::DeleteParameter::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ParameterVersions in a given project, location, and parameter.
    pub fn list_parameter_versions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::ListParameterVersions {
        super::builder::parameter_manager::ListParameterVersions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single ParameterVersion.
    pub fn get_parameter_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::GetParameterVersion {
        super::builder::parameter_manager::GetParameterVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets rendered version of a ParameterVersion.
    pub fn render_parameter_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::RenderParameterVersion {
        super::builder::parameter_manager::RenderParameterVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new ParameterVersion in a given project, location, and parameter.
    pub fn create_parameter_version(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::CreateParameterVersion {
        super::builder::parameter_manager::CreateParameterVersion::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a single ParameterVersion.
    pub fn update_parameter_version(
        &self,
        parameter_version: impl Into<crate::model::ParameterVersion>,
    ) -> super::builder::parameter_manager::UpdateParameterVersion {
        super::builder::parameter_manager::UpdateParameterVersion::new(self.inner.clone())
            .set_parameter_version(parameter_version.into())
    }

    /// Deletes a single ParameterVersion.
    pub fn delete_parameter_version(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::DeleteParameterVersion {
        super::builder::parameter_manager::DeleteParameterVersion::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::ListLocations {
        super::builder::parameter_manager::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parameter_manager::GetLocation {
        super::builder::parameter_manager::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }
}
