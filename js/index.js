import("../pkg/index.js").then(zokrates => {
	window.zokrates = zokrates;
	window.resolve = function resolve_dep(location, path) {
		console.log("Resolving " + path + " from location " + location + " in js");
		return zokrates.ResolverResult.new("def main() -> ():\nreturn", "/path/to/dep/loc")
	};
});