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

use crate::dispatcher;
use crate::types::*;
use std::ptr::{null, null_mut};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::{HostCallError, HostResponseError, Result};

mod abi {
    pub const PROXY_LOG: &str = "proxy_log";
    pub const PROXY_GET_CURRENT_TIME_NANOSECONDS: &str = "proxy_get_current_time_nanoseconds";
    pub const PROXY_SET_TICK_PERIOD_MILLISECONDS: &str = "proxy_set_tick_period_milliseconds";
    pub const PROXY_GET_CONFIGURATION: &str = "proxy_get_configuration";
    pub const PROXY_GET_BUFFER_BYTES: &str = "proxy_get_buffer_bytes";
    pub const PROXY_GET_HEADER_MAP_PAIRS: &str = "proxy_get_header_map_pairs";
    pub const PROXY_SET_HEADER_MAP_PAIRS: &str = "proxy_set_header_map_pairs";
    pub const PROXY_GET_HEADER_MAP_VALUE: &str = "proxy_get_header_map_value";
    pub const PROXY_REPLACE_HEADER_MAP_VALUE: &str = "proxy_replace_header_map_value";
    pub const PROXY_REMOVE_HEADER_MAP_VALUE: &str = "proxy_remove_header_map_value";
    pub const PROXY_ADD_HEADER_MAP_VALUE: &str = "proxy_add_header_map_value";
    pub const PROXY_GET_PROPERTY: &str = "proxy_get_property";
    pub const PROXY_SET_PROPERTY: &str = "proxy_set_property";
    pub const PROXY_GET_SHARED_DATA: &str = "proxy_get_shared_data";
    pub const PROXY_SET_SHARED_DATA: &str = "proxy_set_shared_data";
    pub const PROXY_REGISTER_SHARED_QUEUE: &str = "proxy_register_shared_queue";
    pub const PROXY_RESOLVE_SHARED_QUEUE: &str = "proxy_resolve_shared_queue";
    pub const PROXY_DEQUEUE_SHARED_QUEUE: &str = "proxy_dequeue_shared_queue";
    pub const PROXY_ENQUEUE_SHARED_QUEUE: &str = "proxy_enqueue_shared_queue";
    pub const PROXY_CONTINUE_REQUEST: &str = "proxy_continue_request";
    pub const PROXY_CONTINUE_RESPONSE: &str = "proxy_continue_response";
    pub const PROXY_SEND_LOCAL_RESPONSE: &str = "proxy_send_local_response";
    pub const PROXY_CLEAR_ROUTE_CACHE: &str = "proxy_clear_route_cache";
    pub const PROXY_HTTP_CALL: &str = "proxy_http_call";
    pub const PROXY_SET_EFFECTIVE_CONTEXT: &str = "proxy_set_effective_context";
    pub const PROXY_DONE: &str = "proxy_done";
}

extern "C" {
    fn proxy_log(level: LogLevel, message_data: *const u8, message_size: usize) -> Status;
}

pub fn log(level: LogLevel, message: &str) -> Result<()> {
    unsafe {
        match proxy_log(level, message.as_ptr(), message.len()) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_LOG, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_current_time_nanoseconds(return_time: *mut u64) -> Status;
}

pub fn get_current_time() -> Result<SystemTime> {
    let mut return_time: u64 = 0;
    unsafe {
        match proxy_get_current_time_nanoseconds(&mut return_time) {
            Status::Ok => Ok(UNIX_EPOCH + Duration::from_nanos(return_time)),
            status => {
                Err(HostCallError::new(abi::PROXY_GET_CURRENT_TIME_NANOSECONDS, status).into())
            }
        }
    }
}

extern "C" {
    fn proxy_set_tick_period_milliseconds(period: u32) -> Status;
}

pub fn set_tick_period(period: Duration) -> Result<()> {
    unsafe {
        match proxy_set_tick_period_milliseconds(period.as_millis() as u32) {
            Status::Ok => Ok(()),
            status => {
                Err(HostCallError::new(abi::PROXY_SET_TICK_PERIOD_MILLISECONDS, status).into())
            }
        }
    }
}

extern "C" {
    fn proxy_get_configuration(
        return_buffer_data: *mut *mut u8,
        return_buffer_size: *mut usize,
    ) -> Status;
}

pub fn get_configuration() -> Result<Option<Bytes>> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    unsafe {
        match proxy_get_configuration(&mut return_data, &mut return_size) {
            Status::Ok => {
                if !return_data.is_null() {
                    Ok(Some(Vec::from_raw_parts(
                        return_data,
                        return_size,
                        return_size,
                    )))
                } else {
                    Ok(None)
                }
            }
            status => Err(HostCallError::new(abi::PROXY_GET_CONFIGURATION, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_buffer_bytes(
        buffer_type: BufferType,
        start: usize,
        max_size: usize,
        return_buffer_data: *mut *mut u8,
        return_buffer_size: *mut usize,
    ) -> Status;
}

pub fn get_buffer(buffer_type: BufferType, start: usize, max_size: usize) -> Result<Option<Bytes>> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    unsafe {
        match proxy_get_buffer_bytes(
            buffer_type,
            start,
            max_size,
            &mut return_data,
            &mut return_size,
        ) {
            Status::Ok => {
                if !return_data.is_null() {
                    Ok(Some(Vec::from_raw_parts(
                        return_data,
                        return_size,
                        return_size,
                    )))
                } else {
                    Ok(None)
                }
            }
            Status::NotFound => Ok(None),
            status => Err(HostCallError::new(abi::PROXY_GET_BUFFER_BYTES, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_header_map_pairs(
        map_type: MapType,
        return_map_data: *mut *mut u8,
        return_map_size: *mut usize,
    ) -> Status;
}

pub fn get_map(map_type: MapType) -> Result<Vec<(String, String)>> {
    unsafe {
        let mut return_data: *mut u8 = null_mut();
        let mut return_size: usize = 0;
        match proxy_get_header_map_pairs(map_type, &mut return_data, &mut return_size) {
            Status::Ok => {
                if !return_data.is_null() {
                    let serialized_map = Vec::from_raw_parts(return_data, return_size, return_size);
                    utils::deserialize_map(&serialized_map)
                } else {
                    Ok(Vec::new())
                }
            }
            status => Err(HostCallError::new(abi::PROXY_GET_HEADER_MAP_PAIRS, status).into()),
        }
    }
}

extern "C" {
    fn proxy_set_header_map_pairs(
        map_type: MapType,
        map_data: *const u8,
        map_size: usize,
    ) -> Status;
}

pub fn set_map(map_type: MapType, map: Vec<(&str, &str)>) -> Result<()> {
    let serialized_map = utils::serialize_map(map);
    unsafe {
        match proxy_set_header_map_pairs(map_type, serialized_map.as_ptr(), serialized_map.len()) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_SET_HEADER_MAP_PAIRS, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_header_map_value(
        map_type: MapType,
        key_data: *const u8,
        key_size: usize,
        return_value_data: *mut *mut u8,
        return_value_size: *mut usize,
    ) -> Status;
}

pub fn get_map_value(map_type: MapType, key: &str) -> Result<Option<String>> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    unsafe {
        match proxy_get_header_map_value(
            map_type,
            key.as_ptr(),
            key.len(),
            &mut return_data,
            &mut return_size,
        ) {
            Status::Ok => {
                if !return_data.is_null() {
                    String::from_utf8(Vec::from_raw_parts(return_data, return_size, return_size))
                        .map(Option::from)
                        .map_err(|err| {
                            HostResponseError::new(abi::PROXY_GET_HEADER_MAP_VALUE, err.into())
                                .into()
                        })
                } else {
                    Ok(None)
                }
            }
            status => Err(HostCallError::new(abi::PROXY_GET_HEADER_MAP_VALUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_replace_header_map_value(
        map_type: MapType,
        key_data: *const u8,
        key_size: usize,
        value_data: *const u8,
        value_size: usize,
    ) -> Status;
}

extern "C" {
    fn proxy_remove_header_map_value(
        map_type: MapType,
        key_data: *const u8,
        key_size: usize,
    ) -> Status;
}

pub fn set_map_value(map_type: MapType, key: &str, value: Option<&str>) -> Result<()> {
    unsafe {
        if let Some(value) = value {
            match proxy_replace_header_map_value(
                map_type,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
            ) {
                Status::Ok => Ok(()),
                status => {
                    Err(HostCallError::new(abi::PROXY_REPLACE_HEADER_MAP_VALUE, status).into())
                }
            }
        } else {
            match proxy_remove_header_map_value(map_type, key.as_ptr(), key.len()) {
                Status::Ok => Ok(()),
                status => {
                    Err(HostCallError::new(abi::PROXY_REMOVE_HEADER_MAP_VALUE, status).into())
                }
            }
        }
    }
}

extern "C" {
    fn proxy_add_header_map_value(
        map_type: MapType,
        key_data: *const u8,
        key_size: usize,
        value_data: *const u8,
        value_size: usize,
    ) -> Status;
}

pub fn add_map_value(map_type: MapType, key: &str, value: &str) -> Result<()> {
    unsafe {
        match proxy_add_header_map_value(
            map_type,
            key.as_ptr(),
            key.len(),
            value.as_ptr(),
            value.len(),
        ) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_ADD_HEADER_MAP_VALUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_property(
        path_data: *const u8,
        path_size: usize,
        return_value_data: *mut *mut u8,
        return_value_size: *mut usize,
    ) -> Status;
}

pub fn get_property(path: Vec<&str>) -> Result<Option<Bytes>> {
    let serialized_path = utils::serialize_property_path(path);
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    unsafe {
        match proxy_get_property(
            serialized_path.as_ptr(),
            serialized_path.len(),
            &mut return_data,
            &mut return_size,
        ) {
            Status::Ok => {
                if !return_data.is_null() {
                    Ok(Some(Vec::from_raw_parts(
                        return_data,
                        return_size,
                        return_size,
                    )))
                } else {
                    Ok(None)
                }
            }
            Status::NotFound => Ok(None),
            status => Err(HostCallError::new(abi::PROXY_GET_PROPERTY, status).into()),
        }
    }
}

extern "C" {
    fn proxy_set_property(
        path_data: *const u8,
        path_size: usize,
        value_data: *const u8,
        value_size: usize,
    ) -> Status;
}

pub fn set_property(path: Vec<&str>, value: Option<&[u8]>) -> Result<()> {
    let serialized_path = utils::serialize_property_path(path);
    unsafe {
        match proxy_set_property(
            serialized_path.as_ptr(),
            serialized_path.len(),
            value.map_or(null(), |value| value.as_ptr()),
            value.map_or(0, |value| value.len()),
        ) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_SET_PROPERTY, status).into()),
        }
    }
}

extern "C" {
    fn proxy_get_shared_data(
        key_data: *const u8,
        key_size: usize,
        return_value_data: *mut *mut u8,
        return_value_size: *mut usize,
        return_cas: *mut u32,
    ) -> Status;
}

pub fn get_shared_data(key: &str) -> Result<(Option<Bytes>, Option<u32>)> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    let mut return_cas: u32 = 0;
    unsafe {
        match proxy_get_shared_data(
            key.as_ptr(),
            key.len(),
            &mut return_data,
            &mut return_size,
            &mut return_cas,
        ) {
            Status::Ok => {
                let cas = match return_cas {
                    0 => None,
                    cas => Some(cas),
                };
                if !return_data.is_null() {
                    Ok((
                        Some(Vec::from_raw_parts(return_data, return_size, return_size)),
                        cas,
                    ))
                } else {
                    Ok((None, cas))
                }
            }
            Status::NotFound => Ok((None, None)),
            status => Err(HostCallError::new(abi::PROXY_GET_SHARED_DATA, status).into()),
        }
    }
}

extern "C" {
    fn proxy_set_shared_data(
        key_data: *const u8,
        key_size: usize,
        value_data: *const u8,
        value_size: usize,
        cas: u32,
    ) -> Status;
}

pub fn set_shared_data(key: &str, value: Option<&[u8]>, cas: Option<u32>) -> Result<()> {
    unsafe {
        match proxy_set_shared_data(
            key.as_ptr(),
            key.len(),
            value.map_or(null(), |value| value.as_ptr()),
            value.map_or(0, |value| value.len()),
            cas.unwrap_or(0),
        ) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_SET_SHARED_DATA, status).into()),
        }
    }
}

extern "C" {
    fn proxy_register_shared_queue(
        name_data: *const u8,
        name_size: usize,
        return_id: *mut u32,
    ) -> Status;
}

pub fn register_shared_queue(name: &str) -> Result<u32> {
    unsafe {
        let mut return_id: u32 = 0;
        match proxy_register_shared_queue(name.as_ptr(), name.len(), &mut return_id) {
            Status::Ok => Ok(return_id),
            status => Err(HostCallError::new(abi::PROXY_REGISTER_SHARED_QUEUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_resolve_shared_queue(
        vm_id_data: *const u8,
        vm_id_size: usize,
        name_data: *const u8,
        name_size: usize,
        return_id: *mut u32,
    ) -> Status;
}

pub fn resolve_shared_queue(vm_id: &str, name: &str) -> Result<Option<u32>> {
    let mut return_id: u32 = 0;
    unsafe {
        match proxy_resolve_shared_queue(
            vm_id.as_ptr(),
            vm_id.len(),
            name.as_ptr(),
            name.len(),
            &mut return_id,
        ) {
            Status::Ok => Ok(Some(return_id)),
            Status::NotFound => Ok(None),
            status => Err(HostCallError::new(abi::PROXY_RESOLVE_SHARED_QUEUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_dequeue_shared_queue(
        queue_id: u32,
        return_value_data: *mut *mut u8,
        return_value_size: *mut usize,
    ) -> Status;
}

pub fn dequeue_shared_queue(queue_id: u32) -> Result<Option<Bytes>> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;
    unsafe {
        match proxy_dequeue_shared_queue(queue_id, &mut return_data, &mut return_size) {
            Status::Ok => {
                if !return_data.is_null() {
                    Ok(Some(Vec::from_raw_parts(
                        return_data,
                        return_size,
                        return_size,
                    )))
                } else {
                    Ok(None)
                }
            }
            Status::Empty => Ok(None),
            status => Err(HostCallError::new(abi::PROXY_DEQUEUE_SHARED_QUEUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_enqueue_shared_queue(
        queue_id: u32,
        value_data: *const u8,
        value_size: usize,
    ) -> Status;
}

pub fn enqueue_shared_queue(queue_id: u32, value: Option<&[u8]>) -> Result<()> {
    unsafe {
        match proxy_enqueue_shared_queue(
            queue_id,
            value.map_or(null(), |value| value.as_ptr()),
            value.map_or(0, |value| value.len()),
        ) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_ENQUEUE_SHARED_QUEUE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_continue_request() -> Status;
}

pub fn resume_http_request() -> Result<()> {
    unsafe {
        match proxy_continue_request() {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_CONTINUE_REQUEST, status).into()),
        }
    }
}

extern "C" {
    fn proxy_continue_response() -> Status;
}

pub fn resume_http_response() -> Result<()> {
    unsafe {
        match proxy_continue_response() {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_CONTINUE_RESPONSE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_send_local_response(
        status_code: u32,
        status_code_details_data: *const u8,
        status_code_details_size: usize,
        body_data: *const u8,
        body_size: usize,
        headers_data: *const u8,
        headers_size: usize,
        grpc_status: i32,
    ) -> Status;
}

pub fn send_http_response(
    status_code: u32,
    headers: Vec<(&str, &str)>,
    body: Option<&[u8]>,
) -> Result<()> {
    let serialized_headers = utils::serialize_map(headers);
    unsafe {
        match proxy_send_local_response(
            status_code,
            null(),
            0,
            body.map_or(null(), |body| body.as_ptr()),
            body.map_or(0, |body| body.len()),
            serialized_headers.as_ptr(),
            serialized_headers.len(),
            -1,
        ) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_SEND_LOCAL_RESPONSE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_clear_route_cache() -> Status;
}

pub fn clear_http_route_cache() -> Result<()> {
    unsafe {
        match proxy_clear_route_cache() {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_CLEAR_ROUTE_CACHE, status).into()),
        }
    }
}

extern "C" {
    fn proxy_http_call(
        upstream_data: *const u8,
        upstream_size: usize,
        headers_data: *const u8,
        headers_size: usize,
        body_data: *const u8,
        body_size: usize,
        trailers_data: *const u8,
        trailers_size: usize,
        timeout: u32,
        return_token: *mut u32,
    ) -> Status;
}

pub fn dispatch_http_call(
    upstream: &str,
    headers: Vec<(&str, &str)>,
    body: Option<&[u8]>,
    trailers: Vec<(&str, &str)>,
    timeout: Duration,
) -> Result<u32> {
    let serialized_headers = utils::serialize_map(headers);
    let serialized_trailers = utils::serialize_map(trailers);
    let mut return_token: u32 = 0;
    unsafe {
        match proxy_http_call(
            upstream.as_ptr(),
            upstream.len(),
            serialized_headers.as_ptr(),
            serialized_headers.len(),
            body.map_or(null(), |body| body.as_ptr()),
            body.map_or(0, |body| body.len()),
            serialized_trailers.as_ptr(),
            serialized_trailers.len(),
            timeout.as_millis() as u32,
            &mut return_token,
        ) {
            Status::Ok => {
                dispatcher::register_callout(return_token);
                Ok(return_token)
            }
            status => Err(HostCallError::new(abi::PROXY_HTTP_CALL, status).into()),
        }
    }
}

extern "C" {
    fn proxy_set_effective_context(context_id: u32) -> Status;
}

pub fn set_effective_context(context_id: u32) -> Result<()> {
    unsafe {
        match proxy_set_effective_context(context_id) {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_SET_EFFECTIVE_CONTEXT, status).into()),
        }
    }
}

extern "C" {
    fn proxy_done() -> Status;
}

pub fn done() -> Result<()> {
    unsafe {
        match proxy_done() {
            Status::Ok => Ok(()),
            status => Err(HostCallError::new(abi::PROXY_DONE, status).into()),
        }
    }
}

mod utils {
    use crate::error::Result;
    use crate::types::Bytes;
    use std::convert::TryFrom;

    pub(super) fn serialize_property_path(path: Vec<&str>) -> Bytes {
        if path.is_empty() {
            return Vec::new();
        }
        let mut size: usize = 0;
        for part in &path {
            size += part.len() + 1;
        }
        let mut bytes: Bytes = Vec::with_capacity(size);
        for part in &path {
            bytes.extend_from_slice(&part.as_bytes());
            bytes.push(0);
        }
        bytes.pop();
        bytes
    }

    pub(super) fn serialize_map(map: Vec<(&str, &str)>) -> Bytes {
        let mut size: usize = 4;
        for (name, value) in &map {
            size += name.len() + value.len() + 10;
        }
        let mut bytes: Bytes = Vec::with_capacity(size);
        bytes.extend_from_slice(&map.len().to_le_bytes());
        for (name, value) in &map {
            bytes.extend_from_slice(&name.len().to_le_bytes());
            bytes.extend_from_slice(&value.len().to_le_bytes());
        }
        for (name, value) in &map {
            bytes.extend_from_slice(&name.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&value.as_bytes());
            bytes.push(0);
        }
        bytes
    }

    pub(super) fn deserialize_map(bytes: &[u8]) -> Result<Vec<(String, String)>> {
        let mut map = Vec::new();
        if bytes.is_empty() {
            return Ok(map);
        }
        let size = u32::from_le_bytes(<[u8; 4]>::try_from(&bytes[0..4])?) as usize;
        let mut p = 4 + size * 8;
        for n in 0..size {
            let s = 4 + n * 8;
            let size = u32::from_le_bytes(<[u8; 4]>::try_from(&bytes[s..s + 4])?) as usize;
            let key = bytes[p..p + size].to_vec();
            p += size + 1;
            let size = u32::from_le_bytes(<[u8; 4]>::try_from(&bytes[s + 4..s + 8])?) as usize;
            let value = bytes[p..p + size].to_vec();
            p += size + 1;
            map.push((String::from_utf8(key)?, String::from_utf8(value)?));
        }
        Ok(map)
    }
}
