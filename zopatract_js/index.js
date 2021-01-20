import wrapper from './wrapper.js';
import stdlib from './stdlib.json';
import metadata from './metadata.json';

const initialize = async () => {
  const zopatract = await import('./pkg/index.js');
  return wrapper({ zopatract, stdlib });
}

export { initialize, metadata };