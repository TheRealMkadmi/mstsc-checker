import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const path = require('path');

const mstsc = require(path.join(__dirname, 'index.node'));
export default mstsc;