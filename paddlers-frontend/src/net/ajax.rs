use stdweb::{PromiseFuture};
use stdweb::unstable::TryInto;

pub fn send(method: &str, uri: &str, request_body: &str) -> PromiseFuture<String>{
    let promise: PromiseFuture<String> = 
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
            xhr.send(@{request_body});
        });
    ).try_into().unwrap();

    return promise;
}