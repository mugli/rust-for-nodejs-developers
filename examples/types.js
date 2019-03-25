// primitives
const myBool = true;
const myNumber = 10;
const myString = 'foo';
const mySymbol = Symbol('bar');
const myNull = null;
const myUndefined = undefined;

// object types
const myObject = {};
const myArray = [];
const myFunction = function() {};
const myError = new Error('error');
const myDate = new Date();
const myRegex = /a/;
const myMap = new Map();
const mySet = new Set();
const myPromise = Promise.resolve();
const myGenerator = function*() {};
const myClass = class {};

function makeAdder(x) {
  // JavaScript closure
  return function(y) {
    return x + y;
  };
}

var add5 = makeAdder(5);
var add10 = makeAdder(10);

console.log(add5(2)); // 7
console.log(add10(2)); // 12
