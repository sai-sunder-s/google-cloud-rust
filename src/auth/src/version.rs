// Copyright 2024 Google LLC
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

//! Telemetry header helpers.
mod build_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/build_env.rs"));

    pub(crate) const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
}

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct Version {
    pub(crate) rustc_version: &'static str,
    pub(crate) auth_version: &'static str,
}

impl Version {
    pub(crate) fn to_string(&self) -> String {
        let rustc_version = build_info::RUSTC_VERSION;
        let rustc_version = rustc_version
            .strip_prefix("rustc ")
            .unwrap_or(build_info::RUSTC_VERSION);

        let auth_version = build_info::PKG_VERSION;

        format!("gl-rust/{} auth/{}", rustc_version, auth_version)
    }
}
