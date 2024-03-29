const getAbsolutePath = (basePath, relativePath) => {
    if (relativePath[0] !== '.') {
        return relativePath;
    }
    var stack = basePath.split('/');
    var chunks = relativePath.split('/');
    stack.pop();

    for (var i = 0; i < chunks.length; i++) {
        if (chunks[i] == '.') {
            continue;
        } else if (chunks[i] == '..') {
            stack.pop();
        } else {
            stack.push(chunks[i]);
        }
    }
    return stack.join('/');
}

const getImportPath = (currentLocation, importLocation) => {
    let path = getAbsolutePath(currentLocation, importLocation);
    const extension = path.slice((path.lastIndexOf(".") - 1 >>> 0) + 2);
    return extension ? path : path.concat('.zop');
}

module.exports = (dep) => {

    const { zopatract, stdlib } = dep;

    const resolveFromStdlib = (currentLocation, importLocation) => {
        let key = getImportPath(currentLocation, importLocation);
        let source = stdlib[key];
        return source ? { source, location: key } : null;
    }

    return {
        compile: (source, options = {}) => {
            const { location = "main.zop", resolveCallback = () => null } = options;
            const callback = (currentLocation, importLocation) => {
                return resolveFromStdlib(currentLocation, importLocation) || resolveCallback(currentLocation, importLocation);
            };
            const { program, abi } = zopatract.compile(source, location, callback);
            return {
                program: Array.from(program),
                abi
            }
        },
        setup: (program) => {
            const { vk, pk } = zopatract.setup(program);
            return {
                vk,
                pk: Array.from(pk)
            };
        },
        computeWitness: (artifacts, args) => {
            return zopatract.compute_witness(artifacts, JSON.stringify(Array.from(args)));
        },
        exportSolidityVerifier: (verificationKey, abiVersion) => {
            return zopatract.export_solidity_verifier(verificationKey, abiVersion);
        },
        generateProof: (program, witness, provingKey) => {
            return zopatract.generate_proof(program, witness, provingKey);
        },
        verify: (verificationKey, proof) => {
            return zopatract.verify(verificationKey, proof);
        }
    }
};