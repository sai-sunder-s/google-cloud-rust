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

/// Implements a client for the Cloud Shell API.
///
/// # Service Description
///
/// API for interacting with Google Cloud Shell. Each user of Cloud Shell has at
/// least one environment, which has the ID "default". Environment consists of a
/// Docker image defining what is installed on the environment and a home
/// directory containing the user's data that will remain across sessions.
/// Clients use this API to start and fetch information about their environment,
/// which can then be used to connect to that environment via a separate SSH
/// client.
///
/// # Configuration
///
/// `CloudShellService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `CloudShellService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudShellService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudShellService {
    inner: Arc<dyn super::stub::dynamic::CloudShellService>,
}

impl CloudShellService {
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
        T: super::stub::CloudShellService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::CloudShellService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudShellService> {
        super::transport::CloudShellService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudShellService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudShellService::new)
    }

    /// Gets an environment. Returns NOT_FOUND if the environment does not exist.
    pub fn get_environment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::GetEnvironment {
        super::builder::cloud_shell_service::GetEnvironment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Starts an existing environment, allowing clients to connect to it. The
    /// returned operation will contain an instance of StartEnvironmentMetadata in
    /// its metadata field. Users can wait for the environment to start by polling
    /// this operation via GetOperation. Once the environment has finished starting
    /// and is ready to accept connections, the operation will contain a
    /// StartEnvironmentResponse in its response field.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn start_environment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::StartEnvironment {
        super::builder::cloud_shell_service::StartEnvironment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sends OAuth credentials to a running environment on behalf of a user. When
    /// this completes, the environment will be authorized to run various Google
    /// Cloud command line tools without requiring the user to manually
    /// authenticate.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn authorize_environment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::AuthorizeEnvironment {
        super::builder::cloud_shell_service::AuthorizeEnvironment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Adds a public SSH key to an environment, allowing clients with the
    /// corresponding private key to connect to that environment via SSH. If a key
    /// with the same content already exists, this will error with ALREADY_EXISTS.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn add_public_key(
        &self,
        environment: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::AddPublicKey {
        super::builder::cloud_shell_service::AddPublicKey::new(self.inner.clone())
            .set_environment(environment.into())
    }

    /// Removes a public SSH key from an environment. Clients will no longer be
    /// able to connect to the environment using the corresponding private key.
    /// If a key with the same content is not present, this will error with
    /// NOT_FOUND.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn remove_public_key(
        &self,
        environment: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::RemovePublicKey {
        super::builder::cloud_shell_service::RemovePublicKey::new(self.inner.clone())
            .set_environment(environment.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_shell_service::GetOperation {
        super::builder::cloud_shell_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
