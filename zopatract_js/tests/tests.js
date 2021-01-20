const assert = require('assert');
const { initialize } = require('../node/index.js');

describe('tests', function() {

    // initialize once before running tests
    before(function (done) {
        initialize().then(zopatract => {
            this.zopatract = zopatract;
            done();
        });
    });

    describe("compilation", () => {
        it('should compile', function() {
            assert.doesNotThrow(() => {
                const artifacts = this.zopatract.compile("def main() -> field: return 42");
                assert.ok(artifacts !== undefined);
            })
        });
    
        it('should throw on invalid code', function() {
            assert.throws(() => this.zopatract.compile(":-)"));
        });
    
        it('should resolve stdlib module', function() {
            const stdlib = require('../stdlib.json');
            assert.doesNotThrow(() => {
                const code = `import "${Object.keys(stdlib)[0]}" as func\ndef main(): return`;
                this.zopatract.compile(code);
            });
        });
    
        it('should resolve user module', function() {
            assert.doesNotThrow(() => {
                const code = 'import "test" as test\ndef main() -> field: return test()';
                const options = {
                    resolveCallback: (_, path) => {
                        return {
                            source: "def main() -> (field): return 1",
                            location: path
                        }
                    }
                };
                this.zopatract.compile(code, options);
            });
        });

        it('should throw on unresolved module', function() {
            assert.throws(() => {
                const code = 'import "test" as test\ndef main() -> field: return test()';
                this.zopatract.compile(code);
            });
        });
    });

    describe("computation", () => {
        it('should compute with valid inputs', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
    
                const result = this.zopatract.computeWitness(artifacts, ["2"]);
                const output = JSON.parse(result.output);
    
                assert.deepEqual(output, ["4"]);
            });
        });

        it('should throw on invalid input count', function() {
            assert.throws(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
    
                this.zopatract.computeWitness(artifacts, ["1", "2"]);
            });
        });

        it('should throw on invalid input type', function() {
            assert.throws(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
    
                this.zopatract.computeWitness(artifacts, [true]);
            });
        });
    });

    describe("setup", () => {
        it('should run setup', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
    
                this.zopatract.setup(artifacts.program);
            });
        });
    });

    describe("export-verifier", () => {
        it('should export solidity verifier', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
                const keypair = this.zopatract.setup(artifacts.program);

                const verifier = this.zopatract.exportSolidityVerifier(keypair.vk, "v1");
                assert.ok(verifier.length > 0);
            });
        });
    });

    describe("generate-proof", () => {
        it('should generate proof', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
                const computationResult = this.zopatract.computeWitness(artifacts, ["2"])
                const keypair = this.zopatract.setup(artifacts.program);
                const proof = this.zopatract.generateProof(artifacts.program, computationResult.witness, keypair.pk);

                assert.ok(proof !== undefined);
                assert.deepEqual(proof.inputs, ["0x0000000000000000000000000000000000000000000000000000000000000004"]);
            })
        });
    });

    describe("verify", () => {
        it('should pass', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
                const computationResult = this.zopatract.computeWitness(artifacts, ["2"])
                const keypair = this.zopatract.setup(artifacts.program);
                const proof = this.zopatract.generateProof(artifacts.program, computationResult.witness, keypair.pk);

                assert(this.zopatract.verify(keypair.vk, proof) == true);
            })
        });
        it('should fail', function() {
            assert.doesNotThrow(() => {
                const code = 'def main(private field a) -> field: return a * a';
                const artifacts = this.zopatract.compile(code);
                const computationResult = this.zopatract.computeWitness(artifacts, ["2"])
                const keypair = this.zopatract.setup(artifacts.program);
                let proof = this.zopatract.generateProof(artifacts.program, computationResult.witness, keypair.pk);

                // falsify proof
                proof["proof"]["a"][0] = "0x0000000000000000000000000000000000000000000000000000000000000000";
                assert(this.zopatract.verify(keypair.vk, proof) == false);
            })
        });
    });
});