var square = function(num) {
    return num * 2;
};

var callback = function(call, a, b) {
    return call(a, b);
};

var mult = function(a, b) {
    return a * b;
};

square(callback(mult, 10, 5));
