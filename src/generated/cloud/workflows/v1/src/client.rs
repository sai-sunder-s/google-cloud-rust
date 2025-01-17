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

use crate::Result;
use std::sync::Arc;

/// An implementation of [crate::traits::Workflows] to make requests with.
///
/// `Workflows` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Workflows` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Workflows` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Workflows is used to deploy and execute workflow programs.
/// Workflows makes sure the program executes reliably, despite hardware and
/// networking interruptions.
#[derive(Clone, Debug)]
pub struct Workflows {
    inner: Arc<dyn crate::traits::dyntraits::Workflows>,
}

impl Workflows {
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
        T: crate::traits::Workflows + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::traits::dyntraits::Workflows>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::Workflows> {
        crate::transport::Workflows::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::Workflows> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Workflows::new)
    }

    /// Lists workflows in a given project and location.
    /// The default order is not specified.
    pub fn list_workflows(&self, parent: impl Into<String>) -> crate::builders::ListWorkflows {
        crate::builders::ListWorkflows::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a single workflow.
    pub fn get_workflow(&self, name: impl Into<String>) -> crate::builders::GetWorkflow {
        crate::builders::GetWorkflow::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new workflow. If a workflow with the specified name already
    /// exists in the specified project and location, the long running operation
    /// returns a [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS] error.
    ///
    ///
    /// # Long running operations
    ///
    /// Calling `send()` on the resulting builder starts a longrunning operation.
    /// Long running operations run in the background, and the application may
    /// poll them periodically to find out their completion status.
    ///
    /// To poll the operation use the [get_operation] method. Use the [name]
    /// field in the [Operation] returned from [send()]. When the operation
    /// completes successfully, the [result] field will contain a
    /// [crate::model::Workflow]. If the operation completes with an error it will
    /// contain a `Status` with the error information.
    ///
    /// If the operation is still pending, the [metadata] field will contain a
    /// [crate::model::OperationMetadata]. In many services this provides an indication of
    /// progress.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::CreateWorkflow::send
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    pub fn create_workflow(&self, parent: impl Into<String>) -> crate::builders::CreateWorkflow {
        crate::builders::CreateWorkflow::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Deletes a workflow with the specified name.
    /// This method also cancels and deletes all running executions of the
    /// workflow.
    ///
    /// # Long running operations
    ///
    /// Calling `send()` on the resulting builder starts a longrunning operation.
    /// Long running operations run in the background, and the application may
    /// poll them periodically to find out their completion status.
    ///
    /// To poll the operation use the [get_operation] method. Use the [name]
    /// field in the [Operation] returned from [send()]. When the operation
    /// completes successfully, the [result] field will contain a
    /// [wkt::Empty]. If the operation completes with an error it will
    /// contain a `Status` with the error information.
    ///
    /// If the operation is still pending, the [metadata] field will contain a
    /// [crate::model::OperationMetadata]. In many services this provides an indication of
    /// progress.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::DeleteWorkflow::send
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    pub fn delete_workflow(&self, name: impl Into<String>) -> crate::builders::DeleteWorkflow {
        crate::builders::DeleteWorkflow::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates an existing workflow.
    /// Running this method has no impact on already running executions of the
    /// workflow. A new revision of the workflow might be created as a result of a
    /// successful update operation. In that case, the new revision is used
    /// in new workflow executions.
    ///
    /// # Long running operations
    ///
    /// Calling `send()` on the resulting builder starts a longrunning operation.
    /// Long running operations run in the background, and the application may
    /// poll them periodically to find out their completion status.
    ///
    /// To poll the operation use the [get_operation] method. Use the [name]
    /// field in the [Operation] returned from [send()]. When the operation
    /// completes successfully, the [result] field will contain a
    /// [crate::model::Workflow]. If the operation completes with an error it will
    /// contain a `Status` with the error information.
    ///
    /// If the operation is still pending, the [metadata] field will contain a
    /// [crate::model::OperationMetadata]. In many services this provides an indication of
    /// progress.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::UpdateWorkflow::send
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    pub fn update_workflow(
        &self,
        workflow: impl Into<crate::model::Workflow>,
    ) -> crate::builders::UpdateWorkflow {
        crate::builders::UpdateWorkflow::new(self.inner.clone()).set_workflow(workflow.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self, name: impl Into<String>) -> crate::builders::ListLocations {
        crate::builders::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(&self, name: impl Into<String>) -> crate::builders::GetLocation {
        crate::builders::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    pub fn list_operations(&self, name: impl Into<String>) -> crate::builders::ListOperations {
        crate::builders::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    pub fn get_operation(&self, name: impl Into<String>) -> crate::builders::GetOperation {
        crate::builders::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    pub fn delete_operation(&self, name: impl Into<String>) -> crate::builders::DeleteOperation {
        crate::builders::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }
}
