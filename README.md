## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## What is happening

`js/index.js` defines a resolver as a JS function and makes it available in `window`.

In the browser terminal, execute the following:
```js
zokrates.then(z => z.compile("def main() -> (): return", resolve)).then(console.log)
```

We only illustrate a way for the user to provide a callback for the resolver from the JS side, pretty much everything else is fake here.
