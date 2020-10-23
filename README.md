# techtest
Test task.

Language: Rust

I use it because it`s fast and memory-safe, have many modern features.  

This code shows how the task can be solved with web framework. 
In given repo, I use actix-web framework to implement simple REST API.

Actix-web is extremely fast web framework writed in pure Rust.

Features

    Thread safe async
    Supports HTTP/1.x and HTTP/2
    Streaming and pipelining
    Keep-alive and slow requests handling
    Client/server WebSockets support
    Transparent content compression/decompression (br, gzip, deflate)
    Multipart streams
    Static assets
    SSL support using OpenSSL or Rustls
    Middlewares (Logger, Session, CORS, etc)
    Includes an async HTTP client
    Runs on stable Rust

For verification of code quality I use Rust feature for that.
All main math it task are covered by tests.

How to test.

So, we send the json request to web server, for example:
{"a": false, "b": true, "c": true, "d": 30.0, "e": 10, "f": 10}
to http://127.0.0.1:8080/api/calc
and receive the result.

Result can be json with answer or just error text, if error was occuped.

Using the Insomnia tool it`s can be tested.