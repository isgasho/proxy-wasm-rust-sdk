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

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::traits::*;

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
    HttpCallResponseBody = 4,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MapType {
    HttpRequestHeaders = 0,
    HttpRequestTrailers = 1,
    HttpResponseHeaders = 2,
    HttpResponseTrailers = 3,
    HttpCallResponseHeaders = 6,
    HttpCallResponseTrailers = 7,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PeerType {
    Unknown = 0,
    Local = 1,
    Remote = 2,
}

pub type Bytes = Vec<u8>;

/// Represents an HTTP header value that is not necessarily a UTF-8 encoded string.
#[derive(Eq, Clone)]
pub struct HeaderValue {
    inner: Result<String, Bytes>,
}

impl HeaderValue {
    fn new(inner: Result<String, Bytes>) -> Self {
        HeaderValue { inner }
    }

    pub fn into_vec(self) -> Vec<u8> {
        match self.inner {
            Ok(string) => string.into_bytes(),
            Err(bytes) => bytes,
        }
    }

    pub fn into_string_or_vec(self) -> Result<String, Vec<u8>> {
        self.inner
    }
}

impl From<Vec<u8>> for HeaderValue {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        Self::new(match String::from_utf8(data) {
            Ok(string) => Ok(string),
            Err(err) => Err(err.into_bytes()),
        })
    }
}

impl From<&[u8]> for HeaderValue {
    #[inline]
    fn from(data: &[u8]) -> Self {
        data.to_owned().into()
    }
}

impl From<String> for HeaderValue {
    #[inline]
    fn from(string: String) -> Self {
        Self::new(Ok(string))
    }
}

impl From<&str> for HeaderValue {
    #[inline]
    fn from(data: &str) -> Self {
        data.to_owned().into()
    }
}

impl From<&HeaderValue> for HeaderValue {
    #[inline]
    fn from(data: &HeaderValue) -> Self {
        data.clone()
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Ok(ref string) => fmt::Display::fmt(string, f),
            Err(ref bytes) => fmt::Debug::fmt(bytes, f),
        }
    }
}

impl fmt::Debug for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Ok(ref string) => fmt::Debug::fmt(string, f),
            Err(ref bytes) => fmt::Debug::fmt(bytes, f),
        }
    }
}

impl AsRef<[u8]> for HeaderValue {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self.inner {
            Ok(ref string) => string.as_bytes(),
            Err(ref bytes) => bytes.as_slice(),
        }
    }
}

impl PartialEq for HeaderValue {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<String> for HeaderValue {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other.as_bytes()
    }
}

impl PartialEq<Vec<u8>> for HeaderValue {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.as_ref() == other.as_slice()
    }
}

impl PartialEq<&str> for HeaderValue {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == other.as_bytes()
    }
}

impl PartialEq<&[u8]> for HeaderValue {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<HeaderValue> for String {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}

impl PartialEq<HeaderValue> for Vec<u8> {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}

impl PartialEq<HeaderValue> for &str {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}

impl PartialEq<HeaderValue> for &[u8] {
    #[inline]
    fn eq(&self, other: &HeaderValue) -> bool {
        *other == *self
    }
}

impl Hash for HeaderValue {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.inner {
            Ok(ref string) => Hash::hash(string, state),
            Err(ref bytes) => Hash::hash(bytes, state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_header_value_display_string() {
        let string: HeaderValue = String::from("utf-8 encoded string").into();

        assert_eq!(format!("{}", string), "utf-8 encoded string");
    }

    #[test]
    fn test_header_value_display_bytes() {
        let bytes: HeaderValue = vec![144u8, 145u8, 146u8].into();

        assert_eq!(format!("{}", bytes), "[144, 145, 146]");
    }

    #[test]
    fn test_header_value_debug_string() {
        let string: HeaderValue = String::from("utf-8 encoded string").into();

        assert_eq!(format!("{:?}", string), "\"utf-8 encoded string\"");
    }

    #[test]
    fn test_header_value_debug_bytes() {
        let bytes: HeaderValue = vec![144u8, 145u8, 146u8].into();

        assert_eq!(format!("{:?}", bytes), "[144, 145, 146]");
    }

    #[test]
    fn test_header_value_as_ref() {
        fn receive<T>(value: T)
        where
            T: AsRef<[u8]>,
        {
            value.as_ref();
        }

        let string: HeaderValue = String::from("utf-8 encoded string").into();
        receive(string);

        let bytes: HeaderValue = vec![144u8, 145u8, 146u8].into();
        receive(bytes);
    }

    #[test]
    fn test_header_value_eq_string() {
        let string: HeaderValue = String::from("utf-8 encoded string").into();

        assert_eq!(string, "utf-8 encoded string");
        assert_eq!(string, b"utf-8 encoded string" as &[u8]);

        assert_eq!("utf-8 encoded string", string);
        assert_eq!(b"utf-8 encoded string" as &[u8], string);

        assert_eq!(string, string);
    }

    #[test]
    fn test_header_value_eq_bytes() {
        let bytes: HeaderValue = vec![144u8, 145u8, 146u8].into();

        assert_eq!(bytes, vec![144u8, 145u8, 146u8]);
        assert_eq!(bytes, b"\x90\x91\x92" as &[u8]);

        assert_eq!(vec![144u8, 145u8, 146u8], bytes);
        assert_eq!(b"\x90\x91\x92" as &[u8], bytes);

        assert_eq!(bytes, bytes);
    }

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut h = DefaultHasher::new();
        t.hash(&mut h);
        h.finish()
    }

    #[test]
    fn test_header_value_hash_string() {
        let string: HeaderValue = String::from("utf-8 encoded string").into();

        assert_eq!(hash(&string), hash(&"utf-8 encoded string"));
        assert_ne!(hash(&string), hash(&b"utf-8 encoded string"));

        assert_eq!(hash(&"utf-8 encoded string"), hash(&string));
        assert_ne!(hash(&b"utf-8 encoded string"), hash(&string));
    }

    #[test]
    fn test_header_value_hash_bytes() {
        let bytes: HeaderValue = vec![144u8, 145u8, 146u8].into();

        assert_eq!(hash(&bytes), hash(&vec![144u8, 145u8, 146u8]));
        assert_eq!(hash(&bytes), hash(&[144u8, 145u8, 146u8]));

        assert_eq!(hash(&vec![144u8, 145u8, 146u8]), hash(&bytes));
        assert_eq!(hash(&[144u8, 145u8, 146u8]), hash(&bytes));
    }
}
