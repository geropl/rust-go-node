import { Evaluator } from 'rust-lib-wasm';

let evaluator: Evaluator | undefined;
try {
    evaluator = Evaluator.createFromLicenseKey("", "");
    console.log(`done computing, result: '${evaluator.inspect()}'`);
} finally {
    if (evaluator) {
        evaluator.free();
    }
}