// modules are defined as an array
// [ module function, map of requires ]
//
// map of requires is short require name -> numeric require
//
// anything defined in a previous bundle is accessed via the
// orig method which is the require for previous bundles
parcelRequire = (function (modules, cache, entry, globalName) {
  // Save the require from previous bundle to this closure if any
  var previousRequire = typeof parcelRequire === 'function' && parcelRequire;
  var nodeRequire = typeof require === 'function' && require;

  function newRequire(name, jumped) {
    if (!cache[name]) {
      if (!modules[name]) {
        // if we cannot find the module within our internal map or
        // cache jump to the current global require ie. the last bundle
        // that was added to the page.
        var currentRequire = typeof parcelRequire === 'function' && parcelRequire;
        if (!jumped && currentRequire) {
          return currentRequire(name, true);
        }

        // If there are other bundles on this page the require from the
        // previous one is saved to 'previousRequire'. Repeat this as
        // many times as there are bundles until the module is found or
        // we exhaust the require chain.
        if (previousRequire) {
          return previousRequire(name, true);
        }

        // Try the node require function if it exists.
        if (nodeRequire && typeof name === 'string') {
          return nodeRequire(name);
        }

        var err = new Error('Cannot find module \'' + name + '\'');
        err.code = 'MODULE_NOT_FOUND';
        throw err;
      }

      localRequire.resolve = resolve;
      localRequire.cache = {};

      var module = cache[name] = new newRequire.Module(name);

      modules[name][0].call(module.exports, localRequire, module, module.exports, this);
    }

    return cache[name].exports;

    function localRequire(x){
      return newRequire(localRequire.resolve(x));
    }

    function resolve(x){
      return modules[name][1][x] || x;
    }
  }

  function Module(moduleName) {
    this.id = moduleName;
    this.bundle = newRequire;
    this.exports = {};
  }

  newRequire.isParcelRequire = true;
  newRequire.Module = Module;
  newRequire.modules = modules;
  newRequire.cache = cache;
  newRequire.parent = previousRequire;
  newRequire.register = function (id, exports) {
    modules[id] = [function (require, module) {
      module.exports = exports;
    }, {}];
  };

  var error;
  for (var i = 0; i < entry.length; i++) {
    try {
      newRequire(entry[i]);
    } catch (e) {
      // Save first error but execute all entries
      if (!error) {
        error = e;
      }
    }
  }

  if (entry.length) {
    // Expose entry point to Node, AMD or browser globals
    // Based on https://github.com/ForbesLindesay/umd/blob/master/template.js
    var mainExports = newRequire(entry[entry.length - 1]);

    // CommonJS
    if (typeof exports === "object" && typeof module !== "undefined") {
      module.exports = mainExports;

    // RequireJS
    } else if (typeof define === "function" && define.amd) {
     define(function () {
       return mainExports;
     });

    // <script>
    } else if (globalName) {
      this[globalName] = mainExports;
    }
  }

  // Override the current require with this new one
  parcelRequire = newRequire;

  if (error) {
    // throw error from earlier, _after updating parcelRequire_
    throw error;
  }

  return newRequire;
})({"PfKt":[function(require,module,exports) {
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.__wbindgen_closure_wrapper766 = exports.__wbindgen_closure_wrapper672 = exports.__wbindgen_throw = exports.__wbindgen_debug_string = exports.__wbindgen_string_get = exports.__wbg_set_61642586f7156f4a = exports.__wbg_global_c3c8325ae8c7f1a9 = exports.__wbg_globalThis_f0ca0bbb0149cf3d = exports.__wbg_window_9777ce446d12989f = exports.__wbg_self_05c54dcacb623b9a = exports.__wbg_then_ac66ca61394bfd21 = exports.__wbg_then_367b3e718069cfb9 = exports.__wbg_resolve_778af3f90b8e2b59 = exports.__wbg_new_d14bf16e62c6b3d5 = exports.__wbg_is_573b30cf06a763fb = exports.__wbg_newnoargs_3efc7bfa69a681f9 = exports.__wbg_toString_24c414b45ffa40fa = exports.__wbg_push_17a514d8ab666103 = exports.__wbg_from_c9968c40e2da92d7 = exports.__wbg_new_8528c110a833413f = exports.__wbg_call_cb478d88f3068c91 = exports.__wbg_get_0c6963cbab34fbb6 = exports.__wbg_iterator_de2adb40693c8c47 = exports.__wbg_value_9befa7ab4a7326bf = exports.__wbg_done_faa42c8d1dd8ca9e = exports.__wbg_next_9d10ccb28a5fd327 = exports.__wbg_next_af8c20b8c0d81345 = exports.__wbindgen_is_object = exports.__wbindgen_is_function = exports.__wbg_get_f099d98ea7d68360 = exports.__wbg_newwithstrsequencesequence_fb596b2afa3a5079 = exports.__wbg_search_343a25b176209be0 = exports.__wbg_removeChild_d76a38e31f7ffdcb = exports.__wbg_insertBefore_f2ee50372a21309c = exports.__wbg_appendChild_6ae001e6d3556190 = exports.__wbg_textContent_62903c35b99b5ca3 = exports.__wbg_setnodeValue_eb7b7f2b1e879eec = exports.__wbg_lastChild_41f6d41cb58f88d3 = exports.__wbg_fetch_75f2bada7009e7f1 = exports.__wbg_log_386a8115a84a780d = exports.__wbg_setAttribute_0b50656f1ccc45bf = exports.__wbg_removeAttribute_3b4ea946697b7cea = exports.__wbg_setinnerHTML_bd35babb04d64bb9 = exports.__wbg_namespaceURI_09075ee9acb8b2d7 = exports.__wbg_removeEventListener_1f30d3e3ef4ee58e = exports.__wbg_addEventListener_502683a26945b1a5 = exports.__wbg_abort_13132a8ca4c8ac09 = exports.__wbg_new_94647a205b427932 = exports.__wbg_signal_b959b4cb8b279328 = exports.__wbg_setvalue_7adbd4552719bd8e = exports.__wbg_value_2577d9319a38ca2e = exports.__wbg_settype_6d9bbd4e5c5e8fc9 = exports.__wbg_setchecked_0033386107edc6f2 = exports.__wbg_instanceof_HtmlInputElement_6dfc5638bc87076f = exports.__wbg_settype_b8aa4d6f9b00c6f9 = exports.__wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e = exports.__wbg_setvalue_790f4e4951947e33 = exports.__wbg_value_ad57e46044f59979 = exports.__wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c = exports.__wbg_text_b2095448993eb3f0 = exports.__wbg_arrayBuffer_a98df6d58bb5ea26 = exports.__wbg_headers_f36154094992b8f5 = exports.__wbg_status_5580a898717a7097 = exports.__wbg_newwithstrandinit_a58924208f457f33 = exports.__wbg_querySelector_db4d492deb40e771 = exports.__wbg_createTextNode_278b625a43390ab0 = exports.__wbg_createElementNS_c951238dc260501e = exports.__wbg_createElement_ba61aad8af6be7f4 = exports.__wbg_location_8d4825495dc24c56 = exports.__wbg_fetch_4889502a30fcf1be = exports.__wbg_history_7976f6d5150082fd = exports.__wbg_document_249e9cf340780f93 = exports.__wbg_instanceof_Window_9c4fd26090e1d029 = exports.__wbg_error_4bb6c2a97407129a = exports.__wbg_stack_558ba5917b466edd = exports.__wbg_new_59cb74e423758ede = exports.__wbg_WorkerGlobalScope_65696f271e05e492 = exports.__wbindgen_is_undefined = exports.__wbg_Window_6f26ab8994cdec9b = exports.__wbindgen_object_clone_ref = exports.__wbindgen_string_new = exports.__wbindgen_cb_drop = exports.__wbindgen_object_drop_ref = exports.run = exports.default = void 0;

var _rust_parcel_bg = _interopRequireDefault(require("./pkg/rust_parcel_bg.wasm"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var _default = _rust_parcel_bg.default;
exports.default = _default;
var run = _rust_parcel_bg.default.run;
exports.run = run;
var __wbindgen_object_drop_ref = _rust_parcel_bg.default.__wbindgen_object_drop_ref;
exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;
var __wbindgen_cb_drop = _rust_parcel_bg.default.__wbindgen_cb_drop;
exports.__wbindgen_cb_drop = __wbindgen_cb_drop;
var __wbindgen_string_new = _rust_parcel_bg.default.__wbindgen_string_new;
exports.__wbindgen_string_new = __wbindgen_string_new;
var __wbindgen_object_clone_ref = _rust_parcel_bg.default.__wbindgen_object_clone_ref;
exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;
var __wbg_Window_6f26ab8994cdec9b = _rust_parcel_bg.default.__wbg_Window_6f26ab8994cdec9b;
exports.__wbg_Window_6f26ab8994cdec9b = __wbg_Window_6f26ab8994cdec9b;
var __wbindgen_is_undefined = _rust_parcel_bg.default.__wbindgen_is_undefined;
exports.__wbindgen_is_undefined = __wbindgen_is_undefined;
var __wbg_WorkerGlobalScope_65696f271e05e492 = _rust_parcel_bg.default.__wbg_WorkerGlobalScope_65696f271e05e492;
exports.__wbg_WorkerGlobalScope_65696f271e05e492 = __wbg_WorkerGlobalScope_65696f271e05e492;
var __wbg_new_59cb74e423758ede = _rust_parcel_bg.default.__wbg_new_59cb74e423758ede;
exports.__wbg_new_59cb74e423758ede = __wbg_new_59cb74e423758ede;
var __wbg_stack_558ba5917b466edd = _rust_parcel_bg.default.__wbg_stack_558ba5917b466edd;
exports.__wbg_stack_558ba5917b466edd = __wbg_stack_558ba5917b466edd;
var __wbg_error_4bb6c2a97407129a = _rust_parcel_bg.default.__wbg_error_4bb6c2a97407129a;
exports.__wbg_error_4bb6c2a97407129a = __wbg_error_4bb6c2a97407129a;
var __wbg_instanceof_Window_9c4fd26090e1d029 = _rust_parcel_bg.default.__wbg_instanceof_Window_9c4fd26090e1d029;
exports.__wbg_instanceof_Window_9c4fd26090e1d029 = __wbg_instanceof_Window_9c4fd26090e1d029;
var __wbg_document_249e9cf340780f93 = _rust_parcel_bg.default.__wbg_document_249e9cf340780f93;
exports.__wbg_document_249e9cf340780f93 = __wbg_document_249e9cf340780f93;
var __wbg_history_7976f6d5150082fd = _rust_parcel_bg.default.__wbg_history_7976f6d5150082fd;
exports.__wbg_history_7976f6d5150082fd = __wbg_history_7976f6d5150082fd;
var __wbg_fetch_4889502a30fcf1be = _rust_parcel_bg.default.__wbg_fetch_4889502a30fcf1be;
exports.__wbg_fetch_4889502a30fcf1be = __wbg_fetch_4889502a30fcf1be;
var __wbg_location_8d4825495dc24c56 = _rust_parcel_bg.default.__wbg_location_8d4825495dc24c56;
exports.__wbg_location_8d4825495dc24c56 = __wbg_location_8d4825495dc24c56;
var __wbg_createElement_ba61aad8af6be7f4 = _rust_parcel_bg.default.__wbg_createElement_ba61aad8af6be7f4;
exports.__wbg_createElement_ba61aad8af6be7f4 = __wbg_createElement_ba61aad8af6be7f4;
var __wbg_createElementNS_c951238dc260501e = _rust_parcel_bg.default.__wbg_createElementNS_c951238dc260501e;
exports.__wbg_createElementNS_c951238dc260501e = __wbg_createElementNS_c951238dc260501e;
var __wbg_createTextNode_278b625a43390ab0 = _rust_parcel_bg.default.__wbg_createTextNode_278b625a43390ab0;
exports.__wbg_createTextNode_278b625a43390ab0 = __wbg_createTextNode_278b625a43390ab0;
var __wbg_querySelector_db4d492deb40e771 = _rust_parcel_bg.default.__wbg_querySelector_db4d492deb40e771;
exports.__wbg_querySelector_db4d492deb40e771 = __wbg_querySelector_db4d492deb40e771;
var __wbg_newwithstrandinit_a58924208f457f33 = _rust_parcel_bg.default.__wbg_newwithstrandinit_a58924208f457f33;
exports.__wbg_newwithstrandinit_a58924208f457f33 = __wbg_newwithstrandinit_a58924208f457f33;
var __wbg_status_5580a898717a7097 = _rust_parcel_bg.default.__wbg_status_5580a898717a7097;
exports.__wbg_status_5580a898717a7097 = __wbg_status_5580a898717a7097;
var __wbg_headers_f36154094992b8f5 = _rust_parcel_bg.default.__wbg_headers_f36154094992b8f5;
exports.__wbg_headers_f36154094992b8f5 = __wbg_headers_f36154094992b8f5;
var __wbg_arrayBuffer_a98df6d58bb5ea26 = _rust_parcel_bg.default.__wbg_arrayBuffer_a98df6d58bb5ea26;
exports.__wbg_arrayBuffer_a98df6d58bb5ea26 = __wbg_arrayBuffer_a98df6d58bb5ea26;
var __wbg_text_b2095448993eb3f0 = _rust_parcel_bg.default.__wbg_text_b2095448993eb3f0;
exports.__wbg_text_b2095448993eb3f0 = __wbg_text_b2095448993eb3f0;
var __wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c = _rust_parcel_bg.default.__wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c;
exports.__wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c = __wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c;
var __wbg_value_ad57e46044f59979 = _rust_parcel_bg.default.__wbg_value_ad57e46044f59979;
exports.__wbg_value_ad57e46044f59979 = __wbg_value_ad57e46044f59979;
var __wbg_setvalue_790f4e4951947e33 = _rust_parcel_bg.default.__wbg_setvalue_790f4e4951947e33;
exports.__wbg_setvalue_790f4e4951947e33 = __wbg_setvalue_790f4e4951947e33;
var __wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e = _rust_parcel_bg.default.__wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e;
exports.__wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e = __wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e;
var __wbg_settype_b8aa4d6f9b00c6f9 = _rust_parcel_bg.default.__wbg_settype_b8aa4d6f9b00c6f9;
exports.__wbg_settype_b8aa4d6f9b00c6f9 = __wbg_settype_b8aa4d6f9b00c6f9;
var __wbg_instanceof_HtmlInputElement_6dfc5638bc87076f = _rust_parcel_bg.default.__wbg_instanceof_HtmlInputElement_6dfc5638bc87076f;
exports.__wbg_instanceof_HtmlInputElement_6dfc5638bc87076f = __wbg_instanceof_HtmlInputElement_6dfc5638bc87076f;
var __wbg_setchecked_0033386107edc6f2 = _rust_parcel_bg.default.__wbg_setchecked_0033386107edc6f2;
exports.__wbg_setchecked_0033386107edc6f2 = __wbg_setchecked_0033386107edc6f2;
var __wbg_settype_6d9bbd4e5c5e8fc9 = _rust_parcel_bg.default.__wbg_settype_6d9bbd4e5c5e8fc9;
exports.__wbg_settype_6d9bbd4e5c5e8fc9 = __wbg_settype_6d9bbd4e5c5e8fc9;
var __wbg_value_2577d9319a38ca2e = _rust_parcel_bg.default.__wbg_value_2577d9319a38ca2e;
exports.__wbg_value_2577d9319a38ca2e = __wbg_value_2577d9319a38ca2e;
var __wbg_setvalue_7adbd4552719bd8e = _rust_parcel_bg.default.__wbg_setvalue_7adbd4552719bd8e;
exports.__wbg_setvalue_7adbd4552719bd8e = __wbg_setvalue_7adbd4552719bd8e;
var __wbg_signal_b959b4cb8b279328 = _rust_parcel_bg.default.__wbg_signal_b959b4cb8b279328;
exports.__wbg_signal_b959b4cb8b279328 = __wbg_signal_b959b4cb8b279328;
var __wbg_new_94647a205b427932 = _rust_parcel_bg.default.__wbg_new_94647a205b427932;
exports.__wbg_new_94647a205b427932 = __wbg_new_94647a205b427932;
var __wbg_abort_13132a8ca4c8ac09 = _rust_parcel_bg.default.__wbg_abort_13132a8ca4c8ac09;
exports.__wbg_abort_13132a8ca4c8ac09 = __wbg_abort_13132a8ca4c8ac09;
var __wbg_addEventListener_502683a26945b1a5 = _rust_parcel_bg.default.__wbg_addEventListener_502683a26945b1a5;
exports.__wbg_addEventListener_502683a26945b1a5 = __wbg_addEventListener_502683a26945b1a5;
var __wbg_removeEventListener_1f30d3e3ef4ee58e = _rust_parcel_bg.default.__wbg_removeEventListener_1f30d3e3ef4ee58e;
exports.__wbg_removeEventListener_1f30d3e3ef4ee58e = __wbg_removeEventListener_1f30d3e3ef4ee58e;
var __wbg_namespaceURI_09075ee9acb8b2d7 = _rust_parcel_bg.default.__wbg_namespaceURI_09075ee9acb8b2d7;
exports.__wbg_namespaceURI_09075ee9acb8b2d7 = __wbg_namespaceURI_09075ee9acb8b2d7;
var __wbg_setinnerHTML_bd35babb04d64bb9 = _rust_parcel_bg.default.__wbg_setinnerHTML_bd35babb04d64bb9;
exports.__wbg_setinnerHTML_bd35babb04d64bb9 = __wbg_setinnerHTML_bd35babb04d64bb9;
var __wbg_removeAttribute_3b4ea946697b7cea = _rust_parcel_bg.default.__wbg_removeAttribute_3b4ea946697b7cea;
exports.__wbg_removeAttribute_3b4ea946697b7cea = __wbg_removeAttribute_3b4ea946697b7cea;
var __wbg_setAttribute_0b50656f1ccc45bf = _rust_parcel_bg.default.__wbg_setAttribute_0b50656f1ccc45bf;
exports.__wbg_setAttribute_0b50656f1ccc45bf = __wbg_setAttribute_0b50656f1ccc45bf;
var __wbg_log_386a8115a84a780d = _rust_parcel_bg.default.__wbg_log_386a8115a84a780d;
exports.__wbg_log_386a8115a84a780d = __wbg_log_386a8115a84a780d;
var __wbg_fetch_75f2bada7009e7f1 = _rust_parcel_bg.default.__wbg_fetch_75f2bada7009e7f1;
exports.__wbg_fetch_75f2bada7009e7f1 = __wbg_fetch_75f2bada7009e7f1;
var __wbg_lastChild_41f6d41cb58f88d3 = _rust_parcel_bg.default.__wbg_lastChild_41f6d41cb58f88d3;
exports.__wbg_lastChild_41f6d41cb58f88d3 = __wbg_lastChild_41f6d41cb58f88d3;
var __wbg_setnodeValue_eb7b7f2b1e879eec = _rust_parcel_bg.default.__wbg_setnodeValue_eb7b7f2b1e879eec;
exports.__wbg_setnodeValue_eb7b7f2b1e879eec = __wbg_setnodeValue_eb7b7f2b1e879eec;
var __wbg_textContent_62903c35b99b5ca3 = _rust_parcel_bg.default.__wbg_textContent_62903c35b99b5ca3;
exports.__wbg_textContent_62903c35b99b5ca3 = __wbg_textContent_62903c35b99b5ca3;
var __wbg_appendChild_6ae001e6d3556190 = _rust_parcel_bg.default.__wbg_appendChild_6ae001e6d3556190;
exports.__wbg_appendChild_6ae001e6d3556190 = __wbg_appendChild_6ae001e6d3556190;
var __wbg_insertBefore_f2ee50372a21309c = _rust_parcel_bg.default.__wbg_insertBefore_f2ee50372a21309c;
exports.__wbg_insertBefore_f2ee50372a21309c = __wbg_insertBefore_f2ee50372a21309c;
var __wbg_removeChild_d76a38e31f7ffdcb = _rust_parcel_bg.default.__wbg_removeChild_d76a38e31f7ffdcb;
exports.__wbg_removeChild_d76a38e31f7ffdcb = __wbg_removeChild_d76a38e31f7ffdcb;
var __wbg_search_343a25b176209be0 = _rust_parcel_bg.default.__wbg_search_343a25b176209be0;
exports.__wbg_search_343a25b176209be0 = __wbg_search_343a25b176209be0;
var __wbg_newwithstrsequencesequence_fb596b2afa3a5079 = _rust_parcel_bg.default.__wbg_newwithstrsequencesequence_fb596b2afa3a5079;
exports.__wbg_newwithstrsequencesequence_fb596b2afa3a5079 = __wbg_newwithstrsequencesequence_fb596b2afa3a5079;
var __wbg_get_f099d98ea7d68360 = _rust_parcel_bg.default.__wbg_get_f099d98ea7d68360;
exports.__wbg_get_f099d98ea7d68360 = __wbg_get_f099d98ea7d68360;
var __wbindgen_is_function = _rust_parcel_bg.default.__wbindgen_is_function;
exports.__wbindgen_is_function = __wbindgen_is_function;
var __wbindgen_is_object = _rust_parcel_bg.default.__wbindgen_is_object;
exports.__wbindgen_is_object = __wbindgen_is_object;
var __wbg_next_af8c20b8c0d81345 = _rust_parcel_bg.default.__wbg_next_af8c20b8c0d81345;
exports.__wbg_next_af8c20b8c0d81345 = __wbg_next_af8c20b8c0d81345;
var __wbg_next_9d10ccb28a5fd327 = _rust_parcel_bg.default.__wbg_next_9d10ccb28a5fd327;
exports.__wbg_next_9d10ccb28a5fd327 = __wbg_next_9d10ccb28a5fd327;
var __wbg_done_faa42c8d1dd8ca9e = _rust_parcel_bg.default.__wbg_done_faa42c8d1dd8ca9e;
exports.__wbg_done_faa42c8d1dd8ca9e = __wbg_done_faa42c8d1dd8ca9e;
var __wbg_value_9befa7ab4a7326bf = _rust_parcel_bg.default.__wbg_value_9befa7ab4a7326bf;
exports.__wbg_value_9befa7ab4a7326bf = __wbg_value_9befa7ab4a7326bf;
var __wbg_iterator_de2adb40693c8c47 = _rust_parcel_bg.default.__wbg_iterator_de2adb40693c8c47;
exports.__wbg_iterator_de2adb40693c8c47 = __wbg_iterator_de2adb40693c8c47;
var __wbg_get_0c6963cbab34fbb6 = _rust_parcel_bg.default.__wbg_get_0c6963cbab34fbb6;
exports.__wbg_get_0c6963cbab34fbb6 = __wbg_get_0c6963cbab34fbb6;
var __wbg_call_cb478d88f3068c91 = _rust_parcel_bg.default.__wbg_call_cb478d88f3068c91;
exports.__wbg_call_cb478d88f3068c91 = __wbg_call_cb478d88f3068c91;
var __wbg_new_8528c110a833413f = _rust_parcel_bg.default.__wbg_new_8528c110a833413f;
exports.__wbg_new_8528c110a833413f = __wbg_new_8528c110a833413f;
var __wbg_from_c9968c40e2da92d7 = _rust_parcel_bg.default.__wbg_from_c9968c40e2da92d7;
exports.__wbg_from_c9968c40e2da92d7 = __wbg_from_c9968c40e2da92d7;
var __wbg_push_17a514d8ab666103 = _rust_parcel_bg.default.__wbg_push_17a514d8ab666103;
exports.__wbg_push_17a514d8ab666103 = __wbg_push_17a514d8ab666103;
var __wbg_toString_24c414b45ffa40fa = _rust_parcel_bg.default.__wbg_toString_24c414b45ffa40fa;
exports.__wbg_toString_24c414b45ffa40fa = __wbg_toString_24c414b45ffa40fa;
var __wbg_newnoargs_3efc7bfa69a681f9 = _rust_parcel_bg.default.__wbg_newnoargs_3efc7bfa69a681f9;
exports.__wbg_newnoargs_3efc7bfa69a681f9 = __wbg_newnoargs_3efc7bfa69a681f9;
var __wbg_is_573b30cf06a763fb = _rust_parcel_bg.default.__wbg_is_573b30cf06a763fb;
exports.__wbg_is_573b30cf06a763fb = __wbg_is_573b30cf06a763fb;
var __wbg_new_d14bf16e62c6b3d5 = _rust_parcel_bg.default.__wbg_new_d14bf16e62c6b3d5;
exports.__wbg_new_d14bf16e62c6b3d5 = __wbg_new_d14bf16e62c6b3d5;
var __wbg_resolve_778af3f90b8e2b59 = _rust_parcel_bg.default.__wbg_resolve_778af3f90b8e2b59;
exports.__wbg_resolve_778af3f90b8e2b59 = __wbg_resolve_778af3f90b8e2b59;
var __wbg_then_367b3e718069cfb9 = _rust_parcel_bg.default.__wbg_then_367b3e718069cfb9;
exports.__wbg_then_367b3e718069cfb9 = __wbg_then_367b3e718069cfb9;
var __wbg_then_ac66ca61394bfd21 = _rust_parcel_bg.default.__wbg_then_ac66ca61394bfd21;
exports.__wbg_then_ac66ca61394bfd21 = __wbg_then_ac66ca61394bfd21;
var __wbg_self_05c54dcacb623b9a = _rust_parcel_bg.default.__wbg_self_05c54dcacb623b9a;
exports.__wbg_self_05c54dcacb623b9a = __wbg_self_05c54dcacb623b9a;
var __wbg_window_9777ce446d12989f = _rust_parcel_bg.default.__wbg_window_9777ce446d12989f;
exports.__wbg_window_9777ce446d12989f = __wbg_window_9777ce446d12989f;
var __wbg_globalThis_f0ca0bbb0149cf3d = _rust_parcel_bg.default.__wbg_globalThis_f0ca0bbb0149cf3d;
exports.__wbg_globalThis_f0ca0bbb0149cf3d = __wbg_globalThis_f0ca0bbb0149cf3d;
var __wbg_global_c3c8325ae8c7f1a9 = _rust_parcel_bg.default.__wbg_global_c3c8325ae8c7f1a9;
exports.__wbg_global_c3c8325ae8c7f1a9 = __wbg_global_c3c8325ae8c7f1a9;
var __wbg_set_61642586f7156f4a = _rust_parcel_bg.default.__wbg_set_61642586f7156f4a;
exports.__wbg_set_61642586f7156f4a = __wbg_set_61642586f7156f4a;
var __wbindgen_string_get = _rust_parcel_bg.default.__wbindgen_string_get;
exports.__wbindgen_string_get = __wbindgen_string_get;
var __wbindgen_debug_string = _rust_parcel_bg.default.__wbindgen_debug_string;
exports.__wbindgen_debug_string = __wbindgen_debug_string;
var __wbindgen_throw = _rust_parcel_bg.default.__wbindgen_throw;
exports.__wbindgen_throw = __wbindgen_throw;
var __wbindgen_closure_wrapper672 = _rust_parcel_bg.default.__wbindgen_closure_wrapper672;
exports.__wbindgen_closure_wrapper672 = __wbindgen_closure_wrapper672;
var __wbindgen_closure_wrapper766 = _rust_parcel_bg.default.__wbindgen_closure_wrapper766;
exports.__wbindgen_closure_wrapper766 = __wbindgen_closure_wrapper766;
},{"./pkg/rust_parcel_bg.wasm":"SqEn"}],"QvaY":[function(require,module,exports) {
"use strict";

var _Cargo = _interopRequireDefault(require("../crate/Cargo.toml"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

_Cargo.default.run();
},{"../crate/Cargo.toml":"PfKt"}],"Bh1I":[function(require,module,exports) {
var bundleURL = null;

function getBundleURLCached() {
  if (!bundleURL) {
    bundleURL = getBundleURL();
  }

  return bundleURL;
}

function getBundleURL() {
  // Attempt to find the URL of the current script and use that as the base URL
  try {
    throw new Error();
  } catch (err) {
    var matches = ('' + err.stack).match(/(https?|file|ftp|chrome-extension|moz-extension):\/\/[^)\n]+/g);

    if (matches) {
      return getBaseURL(matches[0]);
    }
  }

  return '/';
}

function getBaseURL(url) {
  return ('' + url).replace(/^((?:https?|file|ftp|chrome-extension|moz-extension):\/\/.+)\/[^/]+$/, '$1') + '/';
}

exports.getBundleURL = getBundleURLCached;
exports.getBaseURL = getBaseURL;
},{}],"z1Am":[function(require,module,exports) {
var getBundleURL = require('./bundle-url').getBundleURL;

function loadBundlesLazy(bundles) {
  if (!Array.isArray(bundles)) {
    bundles = [bundles];
  }

  var id = bundles[bundles.length - 1];

  try {
    return Promise.resolve(require(id));
  } catch (err) {
    if (err.code === 'MODULE_NOT_FOUND') {
      return new LazyPromise(function (resolve, reject) {
        loadBundles(bundles.slice(0, -1)).then(function () {
          return require(id);
        }).then(resolve, reject);
      });
    }

    throw err;
  }
}

function loadBundles(bundles) {
  return Promise.all(bundles.map(loadBundle));
}

var bundleLoaders = {};

function registerBundleLoader(type, loader) {
  bundleLoaders[type] = loader;
}

module.exports = exports = loadBundlesLazy;
exports.load = loadBundles;
exports.register = registerBundleLoader;
var bundles = {};

function loadBundle(bundle) {
  var id;

  if (Array.isArray(bundle)) {
    id = bundle[1];
    bundle = bundle[0];
  }

  if (bundles[bundle]) {
    return bundles[bundle];
  }

  var type = (bundle.substring(bundle.lastIndexOf('.') + 1, bundle.length) || bundle).toLowerCase();
  var bundleLoader = bundleLoaders[type];

  if (bundleLoader) {
    return bundles[bundle] = bundleLoader(getBundleURL() + bundle).then(function (resolved) {
      if (resolved) {
        module.bundle.register(id, resolved);
      }

      return resolved;
    }).catch(function (e) {
      delete bundles[bundle];
      throw e;
    });
  }
}

function LazyPromise(executor) {
  this.executor = executor;
  this.promise = null;
}

LazyPromise.prototype.then = function (onSuccess, onError) {
  if (this.promise === null) this.promise = new Promise(this.executor);
  return this.promise.then(onSuccess, onError);
};

LazyPromise.prototype.catch = function (onError) {
  if (this.promise === null) this.promise = new Promise(this.executor);
  return this.promise.catch(onError);
};
},{"./bundle-url":"Bh1I"}],"sC8V":[function(require,module,exports) {

},{}],"ocK6":[function(require,module,exports) {
var global = arguments[3];
var __dirname = "/home/becker/trash/personal_search/search/node_modules/parcel-plugin-wasm.rs";
var wasm;const __exports = {};

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_22(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h316128fb5bf5a7f9(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha132a687099a3e16(arg0, arg1, addHeapObject(arg2));
}

/**
*/
__exports. run = function() {
    wasm.run();
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

__exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

__exports.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};

__exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

__exports.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

__exports.__wbg_Window_6f26ab8994cdec9b = function(arg0) {
    var ret = getObject(arg0).Window;
    return addHeapObject(ret);
};

__exports.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

__exports.__wbg_WorkerGlobalScope_65696f271e05e492 = function(arg0) {
    var ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
};

__exports.__wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

__exports.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

__exports.__wbg_instanceof_Window_9c4fd26090e1d029 = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

__exports.__wbg_document_249e9cf340780f93 = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_history_7976f6d5150082fd = handleError(function(arg0) {
    var ret = getObject(arg0).history;
    return addHeapObject(ret);
});

__exports.__wbg_fetch_4889502a30fcf1be = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_location_8d4825495dc24c56 = function(arg0) {
    var ret = getObject(arg0).location;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_createElement_ba61aad8af6be7f4 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
});

__exports.__wbg_createElementNS_c951238dc260501e = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    return addHeapObject(ret);
});

__exports.__wbg_createTextNode_278b625a43390ab0 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
};

__exports.__wbg_querySelector_db4d492deb40e771 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

__exports.__wbg_newwithstrandinit_a58924208f457f33 = handleError(function(arg0, arg1, arg2) {
    var ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
    return addHeapObject(ret);
});

__exports.__wbg_status_5580a898717a7097 = function(arg0) {
    var ret = getObject(arg0).status;
    return ret;
};

__exports.__wbg_headers_f36154094992b8f5 = function(arg0) {
    var ret = getObject(arg0).headers;
    return addHeapObject(ret);
};

__exports.__wbg_arrayBuffer_a98df6d58bb5ea26 = handleError(function(arg0) {
    var ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
});

__exports.__wbg_text_b2095448993eb3f0 = handleError(function(arg0) {
    var ret = getObject(arg0).text();
    return addHeapObject(ret);
});

__exports.__wbg_instanceof_HtmlTextAreaElement_aefe0cf650ce9a0c = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLTextAreaElement;
    return ret;
};

__exports.__wbg_value_ad57e46044f59979 = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_setvalue_790f4e4951947e33 = function(arg0, arg1, arg2) {
    getObject(arg0).value = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_instanceof_HtmlButtonElement_98ac0dc8a5eb6f4e = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLButtonElement;
    return ret;
};

__exports.__wbg_settype_b8aa4d6f9b00c6f9 = function(arg0, arg1, arg2) {
    getObject(arg0).type = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_instanceof_HtmlInputElement_6dfc5638bc87076f = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};

__exports.__wbg_setchecked_0033386107edc6f2 = function(arg0, arg1) {
    getObject(arg0).checked = arg1 !== 0;
};

__exports.__wbg_settype_6d9bbd4e5c5e8fc9 = function(arg0, arg1, arg2) {
    getObject(arg0).type = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_value_2577d9319a38ca2e = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_setvalue_7adbd4552719bd8e = function(arg0, arg1, arg2) {
    getObject(arg0).value = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_signal_b959b4cb8b279328 = function(arg0) {
    var ret = getObject(arg0).signal;
    return addHeapObject(ret);
};

__exports.__wbg_new_94647a205b427932 = handleError(function() {
    var ret = new AbortController();
    return addHeapObject(ret);
});

__exports.__wbg_abort_13132a8ca4c8ac09 = function(arg0) {
    getObject(arg0).abort();
};

__exports.__wbg_addEventListener_502683a26945b1a5 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
});

__exports.__wbg_removeEventListener_1f30d3e3ef4ee58e = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
});

__exports.__wbg_namespaceURI_09075ee9acb8b2d7 = function(arg0, arg1) {
    var ret = getObject(arg1).namespaceURI;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_setinnerHTML_bd35babb04d64bb9 = function(arg0, arg1, arg2) {
    getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_removeAttribute_3b4ea946697b7cea = handleError(function(arg0, arg1, arg2) {
    getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
});

__exports.__wbg_setAttribute_0b50656f1ccc45bf = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
});

__exports.__wbg_log_386a8115a84a780d = function(arg0) {
    console.log(getObject(arg0));
};

__exports.__wbg_fetch_75f2bada7009e7f1 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_lastChild_41f6d41cb58f88d3 = function(arg0) {
    var ret = getObject(arg0).lastChild;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_setnodeValue_eb7b7f2b1e879eec = function(arg0, arg1, arg2) {
    getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_textContent_62903c35b99b5ca3 = function(arg0, arg1) {
    var ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_appendChild_6ae001e6d3556190 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_insertBefore_f2ee50372a21309c = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
});

__exports.__wbg_removeChild_d76a38e31f7ffdcb = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_search_343a25b176209be0 = handleError(function(arg0, arg1) {
    var ret = getObject(arg1).search;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

__exports.__wbg_newwithstrsequencesequence_fb596b2afa3a5079 = handleError(function(arg0) {
    var ret = new Headers(getObject(arg0));
    return addHeapObject(ret);
});

__exports.__wbg_get_f099d98ea7d68360 = function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};

__exports.__wbindgen_is_function = function(arg0) {
    var ret = typeof(getObject(arg0)) === 'function';
    return ret;
};

__exports.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    var ret = typeof(val) === 'object' && val !== null;
    return ret;
};

__exports.__wbg_next_af8c20b8c0d81345 = function(arg0) {
    var ret = getObject(arg0).next;
    return addHeapObject(ret);
};

__exports.__wbg_next_9d10ccb28a5fd327 = handleError(function(arg0) {
    var ret = getObject(arg0).next();
    return addHeapObject(ret);
});

__exports.__wbg_done_faa42c8d1dd8ca9e = function(arg0) {
    var ret = getObject(arg0).done;
    return ret;
};

__exports.__wbg_value_9befa7ab4a7326bf = function(arg0) {
    var ret = getObject(arg0).value;
    return addHeapObject(ret);
};

__exports.__wbg_iterator_de2adb40693c8c47 = function() {
    var ret = Symbol.iterator;
    return addHeapObject(ret);
};

__exports.__wbg_get_0c6963cbab34fbb6 = handleError(function(arg0, arg1) {
    var ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_call_cb478d88f3068c91 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_new_8528c110a833413f = function() {
    var ret = new Array();
    return addHeapObject(ret);
};

__exports.__wbg_from_c9968c40e2da92d7 = function(arg0) {
    var ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
};

__exports.__wbg_push_17a514d8ab666103 = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

__exports.__wbg_toString_24c414b45ffa40fa = function(arg0) {
    var ret = getObject(arg0).toString();
    return addHeapObject(ret);
};

__exports.__wbg_newnoargs_3efc7bfa69a681f9 = function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

__exports.__wbg_is_573b30cf06a763fb = function(arg0, arg1) {
    var ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
};

__exports.__wbg_new_d14bf16e62c6b3d5 = function() {
    var ret = new Object();
    return addHeapObject(ret);
};

__exports.__wbg_resolve_778af3f90b8e2b59 = function(arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};

__exports.__wbg_then_367b3e718069cfb9 = function(arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};

__exports.__wbg_then_ac66ca61394bfd21 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_self_05c54dcacb623b9a = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

__exports.__wbg_window_9777ce446d12989f = handleError(function() {
    var ret = window.window;
    return addHeapObject(ret);
});

__exports.__wbg_globalThis_f0ca0bbb0149cf3d = handleError(function() {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
});

__exports.__wbg_global_c3c8325ae8c7f1a9 = handleError(function() {
    var ret = global.global;
    return addHeapObject(ret);
});

__exports.__wbg_set_61642586f7156f4a = handleError(function(arg0, arg1, arg2) {
    var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
});

__exports.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

__exports.__wbindgen_closure_wrapper672 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 280, __wbg_adapter_22);
    return addHeapObject(ret);
};

__exports.__wbindgen_closure_wrapper766 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 333, __wbg_adapter_25);
    return addHeapObject(ret);
};



      function init(wasm_path) {
          const fetchPromise = fetch(wasm_path);
          let resultPromise;
          if (typeof WebAssembly.instantiateStreaming === 'function') {
              resultPromise = WebAssembly.instantiateStreaming(fetchPromise, { './rust_parcel_bg.js': __exports });
          } else {
              resultPromise = fetchPromise
              .then(response => response.arrayBuffer())
              .then(buffer => WebAssembly.instantiate(buffer, { './rust_parcel_bg.js': __exports }));
          }
          return resultPromise.then(({instance}) => {
              wasm = init.wasm = instance.exports;
              __exports.wasm = wasm;
              return;
          });
      };
      function init_node(wasm_path) {
          const fs = require('fs');
          return new Promise(function(resolve, reject) {
              fs.readFile(__dirname + wasm_path, function(err, data) {
                  if (err) {
                      reject(err);
                  } else {
                      resolve(data.buffer);
                  }
              });
          })
          .then(data => WebAssembly.instantiate(data, { './rust_parcel_bg': __exports }))
          .then(({instance}) => {
              wasm = init.wasm = instance.exports;
              __exports.wasm = wasm;
              return;
          });
      }
      const wasm_bindgen = Object.assign(false ? init_node : init, __exports);
      module.exports = function loadWASMBundle(bundle) {
            return wasm_bindgen(bundle).then(() => __exports)
      }
    
},{"fs":"sC8V"}],0:[function(require,module,exports) {
var b=require("z1Am");b.register("wasm",require("ocK6"));b.load([["rust_parcel_bg.f347b42c.wasm","SqEn"]]).then(function(){require("QvaY");});
},{}]},{},[0], null)
//# sourceMappingURL=/js.00a46daa.js.map