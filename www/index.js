import * as wasm from "mbrot";

function $(id) {
  return document.getElementById(id);
}

function focusOnSubmit() {
  var e = $('submitButton');
  if ( e ) e.focus();
}

wasm.greet();
