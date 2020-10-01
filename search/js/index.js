import module from "../crate/Cargo.toml";
module.run();
import Darkmode from "darkmode-js";
new Darkmode().showWidget();

import M from "materialize-css";

document.addEventListener("load", function() {
  var elems = document.querySelectorAll(".modal");
  var instances = M.Modal.init(elems, options);
});
