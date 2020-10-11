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
exports.__wbindgen_closure_wrapper672 = exports.__wbindgen_closure_wrapper525 = exports.__wbindgen_throw = exports.__wbindgen_debug_string = exports.__wbindgen_string_get = exports.__wbg_set_304f2ec1a3ab3b79 = exports.__wbg_global_e16303fe83e1d57f = exports.__wbg_globalThis_b9277fc37e201fe5 = exports.__wbg_window_ba85d88572adc0dc = exports.__wbg_self_07b2f89e82ceb76d = exports.__wbg_then_3b7ac098cfda2fa5 = exports.__wbg_then_4a7a614abbbe6d81 = exports.__wbg_resolve_2529512c3bb73938 = exports.__wbg_new_3e06d4f36713e4cb = exports.__wbg_is_333329c4a02916de = exports.__wbg_newnoargs_f3b8a801d5d4b079 = exports.__wbg_toString_ae4f72e88ee70451 = exports.__wbg_push_b46eeec52d2b03bb = exports.__wbg_from_2a5d647e62275bfd = exports.__wbg_new_e13110f81ae347cf = exports.__wbg_call_8e95613cc6524977 = exports.__wbg_get_0e3f2950cdf758ae = exports.__wbg_iterator_09191f8878ea9877 = exports.__wbg_value_e60bbfb7d52af62f = exports.__wbg_done_037d0a173aef1834 = exports.__wbg_next_2966fa909601a075 = exports.__wbg_next_edda7e0003e5daf9 = exports.__wbindgen_is_object = exports.__wbindgen_is_function = exports.__wbg_get_27693110cb44e852 = exports.__wbg_newwithstrsequencesequence_08147e83d5fd2c0a = exports.__wbg_search_a4c05bf02f7d5a27 = exports.__wbg_removeChild_f4829fcd2e376e1b = exports.__wbg_insertBefore_ea385f1d7ec76e50 = exports.__wbg_appendChild_77215fd672b162c5 = exports.__wbg_textContent_26582d13b9c5ea07 = exports.__wbg_setnodeValue_ec452cd3716fee72 = exports.__wbg_lastChild_4a41600911ce94c7 = exports.__wbg_fetch_8121e255050852c0 = exports.__wbg_log_3bafd82835c6de6d = exports.__wbg_setAttribute_727bdb9763037624 = exports.__wbg_removeAttribute_a705c6de4bb158c4 = exports.__wbg_namespaceURI_a4d52538ca0c6bae = exports.__wbg_removeEventListener_9ffcd175e0916270 = exports.__wbg_addEventListener_f0baf69c9c7425c9 = exports.__wbg_abort_bedcd7330ee18c55 = exports.__wbg_new_a82a2c2e1595c56f = exports.__wbg_signal_0017288388f0d141 = exports.__wbg_setvalue_839acf17e43a847f = exports.__wbg_value_bff6f7ef104e077a = exports.__wbg_settype_4f9e0a597bc38efe = exports.__wbg_setchecked_1a89c058f5ac906a = exports.__wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b = exports.__wbg_newwithstrandinit_d1de1bfcd175e38a = exports.__wbg_settype_98bff8074314478e = exports.__wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00 = exports.__wbg_text_966d07536ca6ccdc = exports.__wbg_arrayBuffer_dc33ab7b8cdf0d63 = exports.__wbg_headers_c736e1fe38752cff = exports.__wbg_status_eb6dbb31556c329f = exports.__wbg_setvalue_1012134a2989f3ee = exports.__wbg_value_91d41b8dbd0b2f0b = exports.__wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92 = exports.__wbg_querySelector_69fd5cd784bcc892 = exports.__wbg_createTextNode_b3c9e3cb02f83ab5 = exports.__wbg_createElementNS_ea14cb45a87a0719 = exports.__wbg_createElement_5bdf88a5af9f17c5 = exports.__wbg_location_9835be61b5ee5ab0 = exports.__wbg_fetch_1893f81c58855a39 = exports.__wbg_document_6cc8d0b87c0a99b9 = exports.__wbg_instanceof_Window_adf3196bdc02b386 = exports.__wbg_error_4bb6c2a97407129a = exports.__wbg_stack_558ba5917b466edd = exports.__wbg_new_59cb74e423758ede = exports.__wbindgen_string_new = exports.__wbg_WorkerGlobalScope_67f1db380a5f80e1 = exports.__wbindgen_is_undefined = exports.__wbg_Window_2b331ff7d706970d = exports.__wbindgen_object_clone_ref = exports.__wbindgen_cb_drop = exports.__wbindgen_object_drop_ref = exports.run = exports.default = void 0;

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
var __wbindgen_object_clone_ref = _rust_parcel_bg.default.__wbindgen_object_clone_ref;
exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;
var __wbg_Window_2b331ff7d706970d = _rust_parcel_bg.default.__wbg_Window_2b331ff7d706970d;
exports.__wbg_Window_2b331ff7d706970d = __wbg_Window_2b331ff7d706970d;
var __wbindgen_is_undefined = _rust_parcel_bg.default.__wbindgen_is_undefined;
exports.__wbindgen_is_undefined = __wbindgen_is_undefined;
var __wbg_WorkerGlobalScope_67f1db380a5f80e1 = _rust_parcel_bg.default.__wbg_WorkerGlobalScope_67f1db380a5f80e1;
exports.__wbg_WorkerGlobalScope_67f1db380a5f80e1 = __wbg_WorkerGlobalScope_67f1db380a5f80e1;
var __wbindgen_string_new = _rust_parcel_bg.default.__wbindgen_string_new;
exports.__wbindgen_string_new = __wbindgen_string_new;
var __wbg_new_59cb74e423758ede = _rust_parcel_bg.default.__wbg_new_59cb74e423758ede;
exports.__wbg_new_59cb74e423758ede = __wbg_new_59cb74e423758ede;
var __wbg_stack_558ba5917b466edd = _rust_parcel_bg.default.__wbg_stack_558ba5917b466edd;
exports.__wbg_stack_558ba5917b466edd = __wbg_stack_558ba5917b466edd;
var __wbg_error_4bb6c2a97407129a = _rust_parcel_bg.default.__wbg_error_4bb6c2a97407129a;
exports.__wbg_error_4bb6c2a97407129a = __wbg_error_4bb6c2a97407129a;
var __wbg_instanceof_Window_adf3196bdc02b386 = _rust_parcel_bg.default.__wbg_instanceof_Window_adf3196bdc02b386;
exports.__wbg_instanceof_Window_adf3196bdc02b386 = __wbg_instanceof_Window_adf3196bdc02b386;
var __wbg_document_6cc8d0b87c0a99b9 = _rust_parcel_bg.default.__wbg_document_6cc8d0b87c0a99b9;
exports.__wbg_document_6cc8d0b87c0a99b9 = __wbg_document_6cc8d0b87c0a99b9;
var __wbg_fetch_1893f81c58855a39 = _rust_parcel_bg.default.__wbg_fetch_1893f81c58855a39;
exports.__wbg_fetch_1893f81c58855a39 = __wbg_fetch_1893f81c58855a39;
var __wbg_location_9835be61b5ee5ab0 = _rust_parcel_bg.default.__wbg_location_9835be61b5ee5ab0;
exports.__wbg_location_9835be61b5ee5ab0 = __wbg_location_9835be61b5ee5ab0;
var __wbg_createElement_5bdf88a5af9f17c5 = _rust_parcel_bg.default.__wbg_createElement_5bdf88a5af9f17c5;
exports.__wbg_createElement_5bdf88a5af9f17c5 = __wbg_createElement_5bdf88a5af9f17c5;
var __wbg_createElementNS_ea14cb45a87a0719 = _rust_parcel_bg.default.__wbg_createElementNS_ea14cb45a87a0719;
exports.__wbg_createElementNS_ea14cb45a87a0719 = __wbg_createElementNS_ea14cb45a87a0719;
var __wbg_createTextNode_b3c9e3cb02f83ab5 = _rust_parcel_bg.default.__wbg_createTextNode_b3c9e3cb02f83ab5;
exports.__wbg_createTextNode_b3c9e3cb02f83ab5 = __wbg_createTextNode_b3c9e3cb02f83ab5;
var __wbg_querySelector_69fd5cd784bcc892 = _rust_parcel_bg.default.__wbg_querySelector_69fd5cd784bcc892;
exports.__wbg_querySelector_69fd5cd784bcc892 = __wbg_querySelector_69fd5cd784bcc892;
var __wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92 = _rust_parcel_bg.default.__wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92;
exports.__wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92 = __wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92;
var __wbg_value_91d41b8dbd0b2f0b = _rust_parcel_bg.default.__wbg_value_91d41b8dbd0b2f0b;
exports.__wbg_value_91d41b8dbd0b2f0b = __wbg_value_91d41b8dbd0b2f0b;
var __wbg_setvalue_1012134a2989f3ee = _rust_parcel_bg.default.__wbg_setvalue_1012134a2989f3ee;
exports.__wbg_setvalue_1012134a2989f3ee = __wbg_setvalue_1012134a2989f3ee;
var __wbg_status_eb6dbb31556c329f = _rust_parcel_bg.default.__wbg_status_eb6dbb31556c329f;
exports.__wbg_status_eb6dbb31556c329f = __wbg_status_eb6dbb31556c329f;
var __wbg_headers_c736e1fe38752cff = _rust_parcel_bg.default.__wbg_headers_c736e1fe38752cff;
exports.__wbg_headers_c736e1fe38752cff = __wbg_headers_c736e1fe38752cff;
var __wbg_arrayBuffer_dc33ab7b8cdf0d63 = _rust_parcel_bg.default.__wbg_arrayBuffer_dc33ab7b8cdf0d63;
exports.__wbg_arrayBuffer_dc33ab7b8cdf0d63 = __wbg_arrayBuffer_dc33ab7b8cdf0d63;
var __wbg_text_966d07536ca6ccdc = _rust_parcel_bg.default.__wbg_text_966d07536ca6ccdc;
exports.__wbg_text_966d07536ca6ccdc = __wbg_text_966d07536ca6ccdc;
var __wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00 = _rust_parcel_bg.default.__wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00;
exports.__wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00 = __wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00;
var __wbg_settype_98bff8074314478e = _rust_parcel_bg.default.__wbg_settype_98bff8074314478e;
exports.__wbg_settype_98bff8074314478e = __wbg_settype_98bff8074314478e;
var __wbg_newwithstrandinit_d1de1bfcd175e38a = _rust_parcel_bg.default.__wbg_newwithstrandinit_d1de1bfcd175e38a;
exports.__wbg_newwithstrandinit_d1de1bfcd175e38a = __wbg_newwithstrandinit_d1de1bfcd175e38a;
var __wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b = _rust_parcel_bg.default.__wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b;
exports.__wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b = __wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b;
var __wbg_setchecked_1a89c058f5ac906a = _rust_parcel_bg.default.__wbg_setchecked_1a89c058f5ac906a;
exports.__wbg_setchecked_1a89c058f5ac906a = __wbg_setchecked_1a89c058f5ac906a;
var __wbg_settype_4f9e0a597bc38efe = _rust_parcel_bg.default.__wbg_settype_4f9e0a597bc38efe;
exports.__wbg_settype_4f9e0a597bc38efe = __wbg_settype_4f9e0a597bc38efe;
var __wbg_value_bff6f7ef104e077a = _rust_parcel_bg.default.__wbg_value_bff6f7ef104e077a;
exports.__wbg_value_bff6f7ef104e077a = __wbg_value_bff6f7ef104e077a;
var __wbg_setvalue_839acf17e43a847f = _rust_parcel_bg.default.__wbg_setvalue_839acf17e43a847f;
exports.__wbg_setvalue_839acf17e43a847f = __wbg_setvalue_839acf17e43a847f;
var __wbg_signal_0017288388f0d141 = _rust_parcel_bg.default.__wbg_signal_0017288388f0d141;
exports.__wbg_signal_0017288388f0d141 = __wbg_signal_0017288388f0d141;
var __wbg_new_a82a2c2e1595c56f = _rust_parcel_bg.default.__wbg_new_a82a2c2e1595c56f;
exports.__wbg_new_a82a2c2e1595c56f = __wbg_new_a82a2c2e1595c56f;
var __wbg_abort_bedcd7330ee18c55 = _rust_parcel_bg.default.__wbg_abort_bedcd7330ee18c55;
exports.__wbg_abort_bedcd7330ee18c55 = __wbg_abort_bedcd7330ee18c55;
var __wbg_addEventListener_f0baf69c9c7425c9 = _rust_parcel_bg.default.__wbg_addEventListener_f0baf69c9c7425c9;
exports.__wbg_addEventListener_f0baf69c9c7425c9 = __wbg_addEventListener_f0baf69c9c7425c9;
var __wbg_removeEventListener_9ffcd175e0916270 = _rust_parcel_bg.default.__wbg_removeEventListener_9ffcd175e0916270;
exports.__wbg_removeEventListener_9ffcd175e0916270 = __wbg_removeEventListener_9ffcd175e0916270;
var __wbg_namespaceURI_a4d52538ca0c6bae = _rust_parcel_bg.default.__wbg_namespaceURI_a4d52538ca0c6bae;
exports.__wbg_namespaceURI_a4d52538ca0c6bae = __wbg_namespaceURI_a4d52538ca0c6bae;
var __wbg_removeAttribute_a705c6de4bb158c4 = _rust_parcel_bg.default.__wbg_removeAttribute_a705c6de4bb158c4;
exports.__wbg_removeAttribute_a705c6de4bb158c4 = __wbg_removeAttribute_a705c6de4bb158c4;
var __wbg_setAttribute_727bdb9763037624 = _rust_parcel_bg.default.__wbg_setAttribute_727bdb9763037624;
exports.__wbg_setAttribute_727bdb9763037624 = __wbg_setAttribute_727bdb9763037624;
var __wbg_log_3bafd82835c6de6d = _rust_parcel_bg.default.__wbg_log_3bafd82835c6de6d;
exports.__wbg_log_3bafd82835c6de6d = __wbg_log_3bafd82835c6de6d;
var __wbg_fetch_8121e255050852c0 = _rust_parcel_bg.default.__wbg_fetch_8121e255050852c0;
exports.__wbg_fetch_8121e255050852c0 = __wbg_fetch_8121e255050852c0;
var __wbg_lastChild_4a41600911ce94c7 = _rust_parcel_bg.default.__wbg_lastChild_4a41600911ce94c7;
exports.__wbg_lastChild_4a41600911ce94c7 = __wbg_lastChild_4a41600911ce94c7;
var __wbg_setnodeValue_ec452cd3716fee72 = _rust_parcel_bg.default.__wbg_setnodeValue_ec452cd3716fee72;
exports.__wbg_setnodeValue_ec452cd3716fee72 = __wbg_setnodeValue_ec452cd3716fee72;
var __wbg_textContent_26582d13b9c5ea07 = _rust_parcel_bg.default.__wbg_textContent_26582d13b9c5ea07;
exports.__wbg_textContent_26582d13b9c5ea07 = __wbg_textContent_26582d13b9c5ea07;
var __wbg_appendChild_77215fd672b162c5 = _rust_parcel_bg.default.__wbg_appendChild_77215fd672b162c5;
exports.__wbg_appendChild_77215fd672b162c5 = __wbg_appendChild_77215fd672b162c5;
var __wbg_insertBefore_ea385f1d7ec76e50 = _rust_parcel_bg.default.__wbg_insertBefore_ea385f1d7ec76e50;
exports.__wbg_insertBefore_ea385f1d7ec76e50 = __wbg_insertBefore_ea385f1d7ec76e50;
var __wbg_removeChild_f4829fcd2e376e1b = _rust_parcel_bg.default.__wbg_removeChild_f4829fcd2e376e1b;
exports.__wbg_removeChild_f4829fcd2e376e1b = __wbg_removeChild_f4829fcd2e376e1b;
var __wbg_search_a4c05bf02f7d5a27 = _rust_parcel_bg.default.__wbg_search_a4c05bf02f7d5a27;
exports.__wbg_search_a4c05bf02f7d5a27 = __wbg_search_a4c05bf02f7d5a27;
var __wbg_newwithstrsequencesequence_08147e83d5fd2c0a = _rust_parcel_bg.default.__wbg_newwithstrsequencesequence_08147e83d5fd2c0a;
exports.__wbg_newwithstrsequencesequence_08147e83d5fd2c0a = __wbg_newwithstrsequencesequence_08147e83d5fd2c0a;
var __wbg_get_27693110cb44e852 = _rust_parcel_bg.default.__wbg_get_27693110cb44e852;
exports.__wbg_get_27693110cb44e852 = __wbg_get_27693110cb44e852;
var __wbindgen_is_function = _rust_parcel_bg.default.__wbindgen_is_function;
exports.__wbindgen_is_function = __wbindgen_is_function;
var __wbindgen_is_object = _rust_parcel_bg.default.__wbindgen_is_object;
exports.__wbindgen_is_object = __wbindgen_is_object;
var __wbg_next_edda7e0003e5daf9 = _rust_parcel_bg.default.__wbg_next_edda7e0003e5daf9;
exports.__wbg_next_edda7e0003e5daf9 = __wbg_next_edda7e0003e5daf9;
var __wbg_next_2966fa909601a075 = _rust_parcel_bg.default.__wbg_next_2966fa909601a075;
exports.__wbg_next_2966fa909601a075 = __wbg_next_2966fa909601a075;
var __wbg_done_037d0a173aef1834 = _rust_parcel_bg.default.__wbg_done_037d0a173aef1834;
exports.__wbg_done_037d0a173aef1834 = __wbg_done_037d0a173aef1834;
var __wbg_value_e60bbfb7d52af62f = _rust_parcel_bg.default.__wbg_value_e60bbfb7d52af62f;
exports.__wbg_value_e60bbfb7d52af62f = __wbg_value_e60bbfb7d52af62f;
var __wbg_iterator_09191f8878ea9877 = _rust_parcel_bg.default.__wbg_iterator_09191f8878ea9877;
exports.__wbg_iterator_09191f8878ea9877 = __wbg_iterator_09191f8878ea9877;
var __wbg_get_0e3f2950cdf758ae = _rust_parcel_bg.default.__wbg_get_0e3f2950cdf758ae;
exports.__wbg_get_0e3f2950cdf758ae = __wbg_get_0e3f2950cdf758ae;
var __wbg_call_8e95613cc6524977 = _rust_parcel_bg.default.__wbg_call_8e95613cc6524977;
exports.__wbg_call_8e95613cc6524977 = __wbg_call_8e95613cc6524977;
var __wbg_new_e13110f81ae347cf = _rust_parcel_bg.default.__wbg_new_e13110f81ae347cf;
exports.__wbg_new_e13110f81ae347cf = __wbg_new_e13110f81ae347cf;
var __wbg_from_2a5d647e62275bfd = _rust_parcel_bg.default.__wbg_from_2a5d647e62275bfd;
exports.__wbg_from_2a5d647e62275bfd = __wbg_from_2a5d647e62275bfd;
var __wbg_push_b46eeec52d2b03bb = _rust_parcel_bg.default.__wbg_push_b46eeec52d2b03bb;
exports.__wbg_push_b46eeec52d2b03bb = __wbg_push_b46eeec52d2b03bb;
var __wbg_toString_ae4f72e88ee70451 = _rust_parcel_bg.default.__wbg_toString_ae4f72e88ee70451;
exports.__wbg_toString_ae4f72e88ee70451 = __wbg_toString_ae4f72e88ee70451;
var __wbg_newnoargs_f3b8a801d5d4b079 = _rust_parcel_bg.default.__wbg_newnoargs_f3b8a801d5d4b079;
exports.__wbg_newnoargs_f3b8a801d5d4b079 = __wbg_newnoargs_f3b8a801d5d4b079;
var __wbg_is_333329c4a02916de = _rust_parcel_bg.default.__wbg_is_333329c4a02916de;
exports.__wbg_is_333329c4a02916de = __wbg_is_333329c4a02916de;
var __wbg_new_3e06d4f36713e4cb = _rust_parcel_bg.default.__wbg_new_3e06d4f36713e4cb;
exports.__wbg_new_3e06d4f36713e4cb = __wbg_new_3e06d4f36713e4cb;
var __wbg_resolve_2529512c3bb73938 = _rust_parcel_bg.default.__wbg_resolve_2529512c3bb73938;
exports.__wbg_resolve_2529512c3bb73938 = __wbg_resolve_2529512c3bb73938;
var __wbg_then_4a7a614abbbe6d81 = _rust_parcel_bg.default.__wbg_then_4a7a614abbbe6d81;
exports.__wbg_then_4a7a614abbbe6d81 = __wbg_then_4a7a614abbbe6d81;
var __wbg_then_3b7ac098cfda2fa5 = _rust_parcel_bg.default.__wbg_then_3b7ac098cfda2fa5;
exports.__wbg_then_3b7ac098cfda2fa5 = __wbg_then_3b7ac098cfda2fa5;
var __wbg_self_07b2f89e82ceb76d = _rust_parcel_bg.default.__wbg_self_07b2f89e82ceb76d;
exports.__wbg_self_07b2f89e82ceb76d = __wbg_self_07b2f89e82ceb76d;
var __wbg_window_ba85d88572adc0dc = _rust_parcel_bg.default.__wbg_window_ba85d88572adc0dc;
exports.__wbg_window_ba85d88572adc0dc = __wbg_window_ba85d88572adc0dc;
var __wbg_globalThis_b9277fc37e201fe5 = _rust_parcel_bg.default.__wbg_globalThis_b9277fc37e201fe5;
exports.__wbg_globalThis_b9277fc37e201fe5 = __wbg_globalThis_b9277fc37e201fe5;
var __wbg_global_e16303fe83e1d57f = _rust_parcel_bg.default.__wbg_global_e16303fe83e1d57f;
exports.__wbg_global_e16303fe83e1d57f = __wbg_global_e16303fe83e1d57f;
var __wbg_set_304f2ec1a3ab3b79 = _rust_parcel_bg.default.__wbg_set_304f2ec1a3ab3b79;
exports.__wbg_set_304f2ec1a3ab3b79 = __wbg_set_304f2ec1a3ab3b79;
var __wbindgen_string_get = _rust_parcel_bg.default.__wbindgen_string_get;
exports.__wbindgen_string_get = __wbindgen_string_get;
var __wbindgen_debug_string = _rust_parcel_bg.default.__wbindgen_debug_string;
exports.__wbindgen_debug_string = __wbindgen_debug_string;
var __wbindgen_throw = _rust_parcel_bg.default.__wbindgen_throw;
exports.__wbindgen_throw = __wbindgen_throw;
var __wbindgen_closure_wrapper525 = _rust_parcel_bg.default.__wbindgen_closure_wrapper525;
exports.__wbindgen_closure_wrapper525 = __wbindgen_closure_wrapper525;
var __wbindgen_closure_wrapper672 = _rust_parcel_bg.default.__wbindgen_closure_wrapper672;
exports.__wbindgen_closure_wrapper672 = __wbindgen_closure_wrapper672;
},{"./pkg/rust_parcel_bg.wasm":"SqEn"}],"iMlC":[function(require,module,exports) {
var define;
(function webpackUniversalModuleDefinition(root, factory) {
	if(typeof exports === 'object' && typeof module === 'object')
		module.exports = factory();
	else if(typeof define === 'function' && define.amd)
		define("darkmode-js", [], factory);
	else if(typeof exports === 'object')
		exports["darkmode-js"] = factory();
	else
		root["darkmode-js"] = factory();
})(typeof self !== 'undefined' ? self : this, function() {
return /******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/darkmode.js":
/*!*************************!*\
  !*** ./src/darkmode.js ***!
  \*************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = exports.IS_BROWSER = void 0;

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } }

function _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); return Constructor; }

var IS_BROWSER = typeof window !== 'undefined';
exports.IS_BROWSER = IS_BROWSER;

var Darkmode =
/*#__PURE__*/
function () {
  function Darkmode(options) {
    _classCallCheck(this, Darkmode);

    if (!IS_BROWSER) {
      return;
    }

    var defaultOptions = {
      bottom: '32px',
      right: '32px',
      left: 'unset',
      time: '0.3s',
      mixColor: '#fff',
      backgroundColor: '#fff',
      buttonColorDark: '#100f2c',
      buttonColorLight: '#fff',
      label: '',
      saveInCookies: true,
      autoMatchOsTheme: true
    };
    options = Object.assign({}, defaultOptions, options);
    var css = "\n      .darkmode-layer {\n        position: fixed;\n        pointer-events: none;\n        background: ".concat(options.mixColor, ";\n        transition: all ").concat(options.time, " ease;\n        mix-blend-mode: difference;\n      }\n\n      .darkmode-layer--button {\n        width: 2.9rem;\n        height: 2.9rem;\n        border-radius: 50%;\n        right: ").concat(options.right, ";\n        bottom: ").concat(options.bottom, ";\n        left: ").concat(options.left, ";\n      }\n\n      .darkmode-layer--simple {\n        width: 100%;\n        height: 100%;\n        top: 0;\n        left: 0;\n        transform: scale(1) !important;\n      }\n\n      .darkmode-layer--expanded {\n        transform: scale(100);\n        border-radius: 0;\n      }\n\n      .darkmode-layer--no-transition {\n        transition: none;\n      }\n\n      .darkmode-toggle {\n        background: ").concat(options.buttonColorDark, ";\n        width: 3rem;\n        height: 3rem;\n        position: fixed;\n        border-radius: 50%;\n        border:none;\n        right: ").concat(options.right, ";\n        bottom: ").concat(options.bottom, ";\n        left: ").concat(options.left, ";\n        cursor: pointer;\n        transition: all 0.5s ease;\n        display: flex;\n        justify-content: center;\n        align-items: center;\n      }\n\n      .darkmode-toggle--white {\n        background: ").concat(options.buttonColorLight, ";\n      }\n\n      .darkmode-toggle--inactive {\n        display: none;\n      }\n\n      .darkmode-background {\n        background: ").concat(options.backgroundColor, ";\n        position: fixed;\n        pointer-events: none;\n        z-index: -10;\n        width: 100%;\n        height: 100%;\n        top: 0;\n        left: 0;\n      }\n\n      img, .darkmode-ignore {\n        isolation: isolate;\n        display: inline-block;\n      }\n\n      @media screen and (-ms-high-contrast: active), (-ms-high-contrast: none) {\n        .darkmode-toggle {display: none !important}\n      }\n\n      @supports (-ms-ime-align:auto), (-ms-accelerator:true) {\n        .darkmode-toggle {display: none !important}\n      }\n    ");
    var layer = document.createElement('div');
    var button = document.createElement('button');
    var background = document.createElement('div');
    button.innerHTML = options.label;
    button.classList.add('darkmode-toggle--inactive');
    layer.classList.add('darkmode-layer');
    background.classList.add('darkmode-background');
    var darkmodeActivated = window.localStorage.getItem('darkmode') === 'true';
    var preferedThemeOs = options.autoMatchOsTheme && window.matchMedia('(prefers-color-scheme: dark)').matches;
    var darkmodeNeverActivatedByAction = window.localStorage.getItem('darkmode') === null;

    if (darkmodeActivated === true && options.saveInCookies || darkmodeNeverActivatedByAction && preferedThemeOs) {
      layer.classList.add('darkmode-layer--expanded', 'darkmode-layer--simple', 'darkmode-layer--no-transition');
      button.classList.add('darkmode-toggle--white');
      document.body.classList.add('darkmode--activated');
    }

    document.body.insertBefore(button, document.body.firstChild);
    document.body.insertBefore(layer, document.body.firstChild);
    document.body.insertBefore(background, document.body.firstChild);
    this.addStyle(css);
    this.button = button;
    this.layer = layer;
    this.saveInCookies = options.saveInCookies;
    this.time = options.time;
  }

  _createClass(Darkmode, [{
    key: "addStyle",
    value: function addStyle(css) {
      var linkElement = document.createElement('link');
      linkElement.setAttribute('rel', 'stylesheet');
      linkElement.setAttribute('type', 'text/css');
      linkElement.setAttribute('href', 'data:text/css;charset=UTF-8,' + encodeURIComponent(css));
      document.head.appendChild(linkElement);
    }
  }, {
    key: "showWidget",
    value: function showWidget() {
      var _this = this;

      if (!IS_BROWSER) {
        return;
      }

      var button = this.button;
      var layer = this.layer;
      var time = parseFloat(this.time) * 1000;
      button.classList.add('darkmode-toggle');
      button.classList.remove('darkmode-toggle--inactive');
      button.setAttribute("aria-label", "Activate dark mode");
      button.setAttribute("aria-checked", "false");
      button.setAttribute("role", "checkbox");
      layer.classList.add('darkmode-layer--button');
      button.addEventListener('click', function () {
        var isDarkmode = _this.isActivated();

        if (!isDarkmode) {
          layer.classList.add('darkmode-layer--expanded');
          button.setAttribute('disabled', true);
          setTimeout(function () {
            layer.classList.add('darkmode-layer--no-transition');
            layer.classList.add('darkmode-layer--simple');
            button.removeAttribute('disabled');
          }, time);
        } else {
          layer.classList.remove('darkmode-layer--simple');
          button.setAttribute('disabled', true);
          setTimeout(function () {
            layer.classList.remove('darkmode-layer--no-transition');
            layer.classList.remove('darkmode-layer--expanded');
            button.removeAttribute('disabled');
          }, 1);
        }

        button.classList.toggle('darkmode-toggle--white');
        document.body.classList.toggle('darkmode--activated');
        window.localStorage.setItem('darkmode', !isDarkmode);
      });
    }
  }, {
    key: "toggle",
    value: function toggle() {
      if (!IS_BROWSER) {
        return;
      }

      var layer = this.layer;
      var isDarkmode = this.isActivated();
      var button = this.button;
      layer.classList.toggle('darkmode-layer--simple');
      document.body.classList.toggle('darkmode--activated');
      window.localStorage.setItem('darkmode', !isDarkmode);
      button.setAttribute("aria-label", "De-activate dark mode");
      button.setAttribute("aria-checked", "true");
    }
  }, {
    key: "isActivated",
    value: function isActivated() {
      if (!IS_BROWSER) {
        return null;
      }

      return document.body.classList.contains('darkmode--activated');
    }
  }]);

  return Darkmode;
}();

exports.default = Darkmode;

/***/ }),

/***/ "./src/index.js":
/*!**********************!*\
  !*** ./src/index.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = void 0;

var _darkmode = _interopRequireWildcard(__webpack_require__(/*! ./darkmode */ "./src/darkmode.js"));

function _interopRequireWildcard(obj) { if (obj && obj.__esModule) { return obj; } else { var newObj = {}; if (obj != null) { for (var key in obj) { if (Object.prototype.hasOwnProperty.call(obj, key)) { var desc = Object.defineProperty && Object.getOwnPropertyDescriptor ? Object.getOwnPropertyDescriptor(obj, key) : {}; if (desc.get || desc.set) { Object.defineProperty(newObj, key, desc); } else { newObj[key] = obj[key]; } } } } newObj.default = obj; return newObj; } }

var _default = _darkmode.default;
/* eslint-disable */

exports.default = _default;

if (_darkmode.IS_BROWSER) {
  (function (window) {
    window.Darkmode = _darkmode.default;
  })(window);
}
/* eslint-enable */


module.exports = exports["default"];

/***/ })

/******/ });
});

},{}],"QvaY":[function(require,module,exports) {
"use strict";

var _Cargo = _interopRequireDefault(require("../crate/Cargo.toml"));

var _darkmodeJs = _interopRequireDefault(require("darkmode-js"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

_Cargo.default.run();

new _darkmodeJs.default().showWidget();
},{"../crate/Cargo.toml":"PfKt","darkmode-js":"iMlC"}],"Bh1I":[function(require,module,exports) {
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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3ff5a5884ee56588(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8b2cb443acfbc990(arg0, arg1, addHeapObject(arg2));
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

__exports.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

__exports.__wbg_Window_2b331ff7d706970d = function(arg0) {
    var ret = getObject(arg0).Window;
    return addHeapObject(ret);
};

__exports.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

__exports.__wbg_WorkerGlobalScope_67f1db380a5f80e1 = function(arg0) {
    var ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
};

__exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
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

__exports.__wbg_instanceof_Window_adf3196bdc02b386 = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

__exports.__wbg_document_6cc8d0b87c0a99b9 = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_fetch_1893f81c58855a39 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_location_9835be61b5ee5ab0 = function(arg0) {
    var ret = getObject(arg0).location;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_createElement_5bdf88a5af9f17c5 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
});

__exports.__wbg_createElementNS_ea14cb45a87a0719 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    return addHeapObject(ret);
});

__exports.__wbg_createTextNode_b3c9e3cb02f83ab5 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
};

__exports.__wbg_querySelector_69fd5cd784bcc892 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

__exports.__wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLTextAreaElement;
    return ret;
};

__exports.__wbg_value_91d41b8dbd0b2f0b = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_setvalue_1012134a2989f3ee = function(arg0, arg1, arg2) {
    getObject(arg0).value = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_status_eb6dbb31556c329f = function(arg0) {
    var ret = getObject(arg0).status;
    return ret;
};

__exports.__wbg_headers_c736e1fe38752cff = function(arg0) {
    var ret = getObject(arg0).headers;
    return addHeapObject(ret);
};

__exports.__wbg_arrayBuffer_dc33ab7b8cdf0d63 = handleError(function(arg0) {
    var ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
});

__exports.__wbg_text_966d07536ca6ccdc = handleError(function(arg0) {
    var ret = getObject(arg0).text();
    return addHeapObject(ret);
});

__exports.__wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLButtonElement;
    return ret;
};

__exports.__wbg_settype_98bff8074314478e = function(arg0, arg1, arg2) {
    getObject(arg0).type = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_newwithstrandinit_d1de1bfcd175e38a = handleError(function(arg0, arg1, arg2) {
    var ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
    return addHeapObject(ret);
});

__exports.__wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};

__exports.__wbg_setchecked_1a89c058f5ac906a = function(arg0, arg1) {
    getObject(arg0).checked = arg1 !== 0;
};

__exports.__wbg_settype_4f9e0a597bc38efe = function(arg0, arg1, arg2) {
    getObject(arg0).type = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_value_bff6f7ef104e077a = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_setvalue_839acf17e43a847f = function(arg0, arg1, arg2) {
    getObject(arg0).value = getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_signal_0017288388f0d141 = function(arg0) {
    var ret = getObject(arg0).signal;
    return addHeapObject(ret);
};

__exports.__wbg_new_a82a2c2e1595c56f = handleError(function() {
    var ret = new AbortController();
    return addHeapObject(ret);
});

__exports.__wbg_abort_bedcd7330ee18c55 = function(arg0) {
    getObject(arg0).abort();
};

__exports.__wbg_addEventListener_f0baf69c9c7425c9 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
});

__exports.__wbg_removeEventListener_9ffcd175e0916270 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
});

__exports.__wbg_namespaceURI_a4d52538ca0c6bae = function(arg0, arg1) {
    var ret = getObject(arg1).namespaceURI;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_removeAttribute_a705c6de4bb158c4 = handleError(function(arg0, arg1, arg2) {
    getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
});

__exports.__wbg_setAttribute_727bdb9763037624 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
});

__exports.__wbg_log_3bafd82835c6de6d = function(arg0) {
    console.log(getObject(arg0));
};

__exports.__wbg_fetch_8121e255050852c0 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_lastChild_4a41600911ce94c7 = function(arg0) {
    var ret = getObject(arg0).lastChild;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

__exports.__wbg_setnodeValue_ec452cd3716fee72 = function(arg0, arg1, arg2) {
    getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

__exports.__wbg_textContent_26582d13b9c5ea07 = function(arg0, arg1) {
    var ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

__exports.__wbg_appendChild_77215fd672b162c5 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_insertBefore_ea385f1d7ec76e50 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
});

__exports.__wbg_removeChild_f4829fcd2e376e1b = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_search_a4c05bf02f7d5a27 = handleError(function(arg0, arg1) {
    var ret = getObject(arg1).search;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

__exports.__wbg_newwithstrsequencesequence_08147e83d5fd2c0a = handleError(function(arg0) {
    var ret = new Headers(getObject(arg0));
    return addHeapObject(ret);
});

__exports.__wbg_get_27693110cb44e852 = function(arg0, arg1) {
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

__exports.__wbg_next_edda7e0003e5daf9 = function(arg0) {
    var ret = getObject(arg0).next;
    return addHeapObject(ret);
};

__exports.__wbg_next_2966fa909601a075 = handleError(function(arg0) {
    var ret = getObject(arg0).next();
    return addHeapObject(ret);
});

__exports.__wbg_done_037d0a173aef1834 = function(arg0) {
    var ret = getObject(arg0).done;
    return ret;
};

__exports.__wbg_value_e60bbfb7d52af62f = function(arg0) {
    var ret = getObject(arg0).value;
    return addHeapObject(ret);
};

__exports.__wbg_iterator_09191f8878ea9877 = function() {
    var ret = Symbol.iterator;
    return addHeapObject(ret);
};

__exports.__wbg_get_0e3f2950cdf758ae = handleError(function(arg0, arg1) {
    var ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_call_8e95613cc6524977 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
});

__exports.__wbg_new_e13110f81ae347cf = function() {
    var ret = new Array();
    return addHeapObject(ret);
};

__exports.__wbg_from_2a5d647e62275bfd = function(arg0) {
    var ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
};

__exports.__wbg_push_b46eeec52d2b03bb = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

__exports.__wbg_toString_ae4f72e88ee70451 = function(arg0) {
    var ret = getObject(arg0).toString();
    return addHeapObject(ret);
};

__exports.__wbg_newnoargs_f3b8a801d5d4b079 = function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

__exports.__wbg_is_333329c4a02916de = function(arg0, arg1) {
    var ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
};

__exports.__wbg_new_3e06d4f36713e4cb = function() {
    var ret = new Object();
    return addHeapObject(ret);
};

__exports.__wbg_resolve_2529512c3bb73938 = function(arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};

__exports.__wbg_then_4a7a614abbbe6d81 = function(arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};

__exports.__wbg_then_3b7ac098cfda2fa5 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

__exports.__wbg_self_07b2f89e82ceb76d = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

__exports.__wbg_window_ba85d88572adc0dc = handleError(function() {
    var ret = window.window;
    return addHeapObject(ret);
});

__exports.__wbg_globalThis_b9277fc37e201fe5 = handleError(function() {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
});

__exports.__wbg_global_e16303fe83e1d57f = handleError(function() {
    var ret = global.global;
    return addHeapObject(ret);
});

__exports.__wbg_set_304f2ec1a3ab3b79 = handleError(function(arg0, arg1, arg2) {
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

__exports.__wbindgen_closure_wrapper525 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 203, __wbg_adapter_22);
    return addHeapObject(ret);
};

__exports.__wbindgen_closure_wrapper672 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 272, __wbg_adapter_25);
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