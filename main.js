import init, { run_app } from './pkg/todomvc.js';
async function main() {
   await init('/pkg/todomvc_bg.wasm');
   run_app();
}
main()