// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::traits::*;

pub use crate::bytestring::ByteString;

pub type NewRootContext = fn(context_id: u32) -> Box<dyn RootContext>;
pub type NewStreamContext = fn(context_id: u32, root_context_id: u32) -> Box<dyn StreamContext>;
pub type NewHttpContext = fn(context_id: u32, root_context_id: u32) -> Box<dyn HttpContext>;

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Action {
    Continue = 0,
    Pause = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Status {
    Ok = 0,
    NotFound = 1,
    BadArgument = 2,
    Empty = 7,
    CasMismatch = 8,
    InternalFailure = 10,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BufferType {
    HttpRequestBody = 0,
    HttpResponseBody = 1,
    DownstreamData = 2,
    UpstreamData = 3,
    HttpCallResponseBody = 4, // Immutable
    GrpcReceiveBuffer = 5,    // Immutable
    VmConfiguration = 6,      // Immutable
    PluginConfiguration = 7,  // Immutable
    CallData = 8,             // Immutable
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MapType {
    HttpRequestHeaders = 0,
    HttpRequestTrailers = 1,
    HttpResponseHeaders = 2,
    HttpResponseTrailers = 3,
    GrpcReceiveInitialMetadata = 4,  // Immutable
    GrpcReceiveTrailingMetadata = 5, // Immutable
    HttpCallResponseHeaders = 6,     // Immutable
    HttpCallResponseTrailers = 7,    // Immutable
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PeerType {
    Unknown = 0,
    Local = 1,
    Remote = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StreamType {
    Request = 0,
    Response = 1,
}
