# techtest
Test task.

This code shows how the task can be solved with web framework. I use actix-web framework to implement simple REST API with only one link /api/calc.
So, we send the json request to web server, for example:
{"a": false, "b": true, "c": true, "d": 30.0, "e": 10, "f": 10}
to http://127.0.0.1:8080/api/calc
and receive the result.

Result can be like json this answer or just error text, if error was occuped.

Using the Insomnia tool it`s can be tested.