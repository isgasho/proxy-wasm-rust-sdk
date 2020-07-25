var searchIndex = JSON.parse('{\
"proxy_wasm_experimental":{"doc":"","i":[[5,"set_log_level","proxy_wasm_experimental","",null,[[["loglevel",4]]]],[5,"set_root_context","","",null,[[]]],[5,"set_stream_context","","",null,[[]]],[5,"set_http_context","","",null,[[]]],[5,"proxy_abi_version_0_1_0","","",null,[[]]],[0,"error","","",null,null],[3,"HostCallError","proxy_wasm_experimental::error","An error to call a Host ABI function.",null,null],[3,"HostResponseError","","An error to parse the response from a Host ABI.",null,null],[6,"Error","","A boxed [`Error`].",null,null],[6,"Result","","A specialized [`Result`] type.",null,null],[11,"module","","",0,[[]]],[11,"function","","",0,[[]]],[11,"status","","",0,[[],["status",4]]],[11,"module","","",1,[[]]],[11,"function","","",1,[[]]],[0,"hostcalls","proxy_wasm_experimental","",null,null],[5,"log","proxy_wasm_experimental::hostcalls","",null,[[["loglevel",4]],["result",6]]],[5,"get_current_time","","",null,[[],[["systemtime",3],["result",6]]]],[5,"set_tick_period","","",null,[[["duration",3]],["result",6]]],[5,"get_configuration","","",null,[[],[["result",6],["option",4]]]],[5,"get_buffer","","",null,[[["buffertype",4]],[["result",6],["option",4]]]],[5,"get_map","","",null,[[["maptype",4]],[["vec",3],["result",6]]]],[5,"set_map","","",null,[[["maptype",4],["vec",3]],["result",6]]],[5,"get_map_value","","",null,[[["maptype",4]],[["result",6],["option",4]]]],[5,"set_map_value","","",null,[[["maptype",4],["option",4]],["result",6]]],[5,"add_map_value","","",null,[[["maptype",4]],["result",6]]],[5,"get_property","","",null,[[["vec",3]],[["result",6],["option",4]]]],[5,"set_property","","",null,[[["option",4],["vec",3]],["result",6]]],[5,"get_shared_data","","",null,[[],["result",6]]],[5,"set_shared_data","","",null,[[["option",4],["option",4]],["result",6]]],[5,"register_shared_queue","","",null,[[],["result",6]]],[5,"resolve_shared_queue","","",null,[[],[["option",4],["result",6]]]],[5,"dequeue_shared_queue","","",null,[[],[["result",6],["option",4]]]],[5,"enqueue_shared_queue","","",null,[[["option",4]],["result",6]]],[5,"resume_http_request","","",null,[[],["result",6]]],[5,"resume_http_response","","",null,[[],["result",6]]],[5,"send_http_response","","",null,[[["option",4],["vec",3]],["result",6]]],[5,"clear_http_route_cache","","",null,[[],["result",6]]],[5,"dispatch_http_call","","",null,[[["option",4],["duration",3],["vec",3]],["result",6]]],[5,"set_effective_context","","",null,[[],["result",6]]],[5,"done","","",null,[[],["result",6]]],[0,"traits","proxy_wasm_experimental","",null,null],[4,"ChildContext","proxy_wasm_experimental::traits","Represents a child context of the root context.",null,null],[13,"StreamContext","","",2,null],[13,"HttpContext","","",2,null],[8,"Context","","",null,null],[11,"get_current_time","","",3,[[],["systemtime",3]]],[11,"get_property","","",3,[[["vec",3]],[["bytes",6],["option",4]]]],[11,"set_property","","",3,[[["option",4],["vec",3]]]],[11,"get_shared_data","","",3,[[]]],[11,"set_shared_data","","",3,[[["option",4],["option",4]],["result",6]]],[11,"register_shared_queue","","",3,[[]]],[11,"resolve_shared_queue","","",3,[[],["option",4]]],[11,"dequeue_shared_queue","","",3,[[],[["result",6],["option",4]]]],[11,"enqueue_shared_queue","","",3,[[["option",4]],["result",6]]],[11,"dispatch_http_call","","",3,[[["option",4],["duration",3],["vec",3]],["result",6]]],[11,"on_http_call_response","","",3,[[]]],[11,"get_http_call_response_headers","","",3,[[],["vec",3]]],[11,"get_http_call_response_body","","",3,[[],[["bytes",6],["option",4]]]],[11,"get_http_call_response_trailers","","",3,[[],["vec",3]]],[11,"on_done","","",3,[[]]],[11,"done","","",3,[[]]],[8,"RootContext","","",null,null],[11,"on_vm_start","","",4,[[]]],[11,"on_configure","","",4,[[]]],[11,"get_configuration","","",4,[[],[["bytes",6],["option",4]]]],[11,"set_tick_period","","",4,[[["duration",3]]]],[11,"on_tick","","",4,[[]]],[11,"on_queue_ready","","",4,[[]]],[11,"on_log","","",4,[[]]],[11,"on_create_child_context","","",4,[[],[["option",4],["childcontext",4]]]],[8,"StreamContext","","",null,null],[11,"on_new_connection","","",5,[[],["action",4]]],[11,"on_downstream_data","","",5,[[],["action",4]]],[11,"get_downstream_data","","",5,[[],[["bytes",6],["option",4]]]],[11,"on_downstream_close","","",5,[[["peertype",4]]]],[11,"on_upstream_data","","",5,[[],["action",4]]],[11,"get_upstream_data","","",5,[[],[["bytes",6],["option",4]]]],[11,"on_upstream_close","","",5,[[["peertype",4]]]],[11,"on_log","","",5,[[]]],[8,"HttpContext","","",null,null],[11,"on_http_request_headers","","",6,[[],["action",4]]],[11,"get_http_request_headers","","",6,[[],["vec",3]]],[11,"set_http_request_headers","","",6,[[["vec",3]]]],[11,"get_http_request_header","","",6,[[],[["option",4],["string",3]]]],[11,"set_http_request_header","","",6,[[["option",4]]]],[11,"add_http_request_header","","",6,[[]]],[11,"on_http_request_body","","",6,[[],["action",4]]],[11,"get_http_request_body","","",6,[[],[["bytes",6],["option",4]]]],[11,"on_http_request_trailers","","",6,[[],["action",4]]],[11,"get_http_request_trailers","","",6,[[],["vec",3]]],[11,"set_http_request_trailers","","",6,[[["vec",3]]]],[11,"get_http_request_trailer","","",6,[[],[["option",4],["string",3]]]],[11,"set_http_request_trailer","","",6,[[["option",4]]]],[11,"add_http_request_trailer","","",6,[[]]],[11,"resume_http_request","","",6,[[]]],[11,"on_http_response_headers","","",6,[[],["action",4]]],[11,"get_http_response_headers","","",6,[[],["vec",3]]],[11,"set_http_response_headers","","",6,[[["vec",3]]]],[11,"get_http_response_header","","",6,[[],[["option",4],["string",3]]]],[11,"set_http_response_header","","",6,[[["option",4]]]],[11,"add_http_response_header","","",6,[[]]],[11,"on_http_response_body","","",6,[[],["action",4]]],[11,"get_http_response_body","","",6,[[],[["bytes",6],["option",4]]]],[11,"on_http_response_trailers","","",6,[[],["action",4]]],[11,"get_http_response_trailers","","",6,[[],["vec",3]]],[11,"set_http_response_trailers","","",6,[[["vec",3]]]],[11,"get_http_response_trailer","","",6,[[],[["option",4],["string",3]]]],[11,"set_http_response_trailer","","",6,[[["option",4]]]],[11,"add_http_response_trailer","","",6,[[]]],[11,"resume_http_response","","",6,[[]]],[11,"send_http_response","","",6,[[["option",4],["vec",3]]]],[11,"clear_http_route_cache","","",6,[[]]],[11,"on_log","","",6,[[]]],[0,"types","proxy_wasm_experimental","",null,null],[4,"LogLevel","proxy_wasm_experimental::types","",null,null],[13,"Trace","","",7,null],[13,"Debug","","",7,null],[13,"Info","","",7,null],[13,"Warn","","",7,null],[13,"Error","","",7,null],[13,"Critical","","",7,null],[4,"Action","","",null,null],[13,"Continue","","",8,null],[13,"Pause","","",8,null],[4,"Status","","",null,null],[13,"Ok","","",9,null],[13,"NotFound","","",9,null],[13,"BadArgument","","",9,null],[13,"Empty","","",9,null],[13,"CasMismatch","","",9,null],[13,"InternalFailure","","",9,null],[4,"BufferType","","",null,null],[13,"HttpRequestBody","","",10,null],[13,"HttpResponseBody","","",10,null],[13,"DownstreamData","","",10,null],[13,"UpstreamData","","",10,null],[13,"HttpCallResponseBody","","",10,null],[4,"MapType","","",null,null],[13,"HttpRequestHeaders","","",11,null],[13,"HttpRequestTrailers","","",11,null],[13,"HttpResponseHeaders","","",11,null],[13,"HttpResponseTrailers","","",11,null],[13,"HttpCallResponseHeaders","","",11,null],[13,"HttpCallResponseTrailers","","",11,null],[4,"PeerType","","",null,null],[13,"Unknown","","",12,null],[13,"Local","","",12,null],[13,"Remote","","",12,null],[6,"NewRootContext","","",null,null],[6,"NewStreamContext","","",null,null],[6,"NewHttpContext","","",null,null],[6,"Bytes","","",null,null],[11,"from","proxy_wasm_experimental::error","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_string","","",0,[[],["string",3]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","proxy_wasm_experimental::traits","",2,[[]]],[11,"into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","proxy_wasm_experimental::types","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"clone","","",7,[[],["loglevel",4]]],[11,"clone","","",8,[[],["action",4]]],[11,"clone","","",9,[[],["status",4]]],[11,"clone","","",10,[[],["buffertype",4]]],[11,"clone","","",11,[[],["maptype",4]]],[11,"clone","","",12,[[],["peertype",4]]],[11,"eq","","",7,[[["loglevel",4]]]],[11,"eq","","",8,[[["action",4]]]],[11,"eq","","",9,[[["status",4]]]],[11,"eq","","",10,[[["buffertype",4]]]],[11,"eq","","",11,[[["maptype",4]]]],[11,"eq","","",12,[[["peertype",4]]]],[11,"fmt","proxy_wasm_experimental::error","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","proxy_wasm_experimental::types","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","proxy_wasm_experimental::error","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"hash","proxy_wasm_experimental::types","",7,[[]]],[11,"hash","","",8,[[]]],[11,"hash","","",9,[[]]],[11,"hash","","",10,[[]]],[11,"hash","","",11,[[]]],[11,"hash","","",12,[[]]],[11,"source","proxy_wasm_experimental::error","",1,[[],[["error",8],["option",4]]]]],"p":[[3,"HostCallError"],[3,"HostResponseError"],[4,"ChildContext"],[8,"Context"],[8,"RootContext"],[8,"StreamContext"],[8,"HttpContext"],[4,"LogLevel"],[4,"Action"],[4,"Status"],[4,"BufferType"],[4,"MapType"],[4,"PeerType"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);