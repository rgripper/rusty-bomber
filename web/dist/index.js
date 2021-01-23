// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
import '../../target/wasm'


// const rust = import('../target/wasm');

// rust
//     .then(m => {
//         console.log(m)
//         return m.default()
//     })
//     .catch(console.error);