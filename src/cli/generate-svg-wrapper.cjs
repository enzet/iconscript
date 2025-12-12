#!/usr/bin/env node

// CommonJS wrapper for the ES module CLI.
// This is needed because npm's binary handling for ES modules can be problematic.
const {spawn} = require('child_process');
const path = require('path');

const wrapperPath = __filename;
const esmPath = path.join(path.dirname(wrapperPath), 'generate-svg.js');

const child = spawn(process.execPath, [esmPath, ...process.argv.slice(2)], {
    stdio: 'inherit',
    cwd: process.cwd(),
});

child.on('exit', (code) => {
    process.exit(code || 0);
});

