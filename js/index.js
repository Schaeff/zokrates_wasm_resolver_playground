window.resolve = function resolve_dep(v) {
	console.log("Resolving " + v + " from js");
	return "def dependency() -> (): return";
};

window.zokrates = import("../pkg/index.js").catch(console.error);
