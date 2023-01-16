import { createRequire } from 'module';
const require = createRequire(import.meta.url);

const mstsc = require('./index.node');
export default mstsc;