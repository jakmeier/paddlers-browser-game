use stdweb::{PromiseFuture};
use stdweb::unstable::TryInto;
use crate::prelude::*;

pub fn send(method: &str, uri: &str, request_body: &str) -> PadlResult<PromiseFuture<String>>{
    let promise: Result<PromiseFuture<String>, _> = 
    js! (
        return new Promise(function (resolve, reject) {
            var xhr = new XMLHttpRequest();
            xhr.onload = function() {
                if (xhr.status == 200) {
                    resolve(xhr.response);
                } else {
                    console.log("XHR failed");
                    reject(new Error(xhr.statusText));
                }
            };
            xhr.open(@{method}, @{uri});
            xhr.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
            xhr.onerror = reject;
            // xhr.onerror = function() {console.log("Error handled in JS")};
            xhr.send(@{request_body});
        });
    ).try_into();

    return promise.map_err(PadlError::from);
}