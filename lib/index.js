let addon = require('../native');

/**
 * hello function
 *
 * @callback helloFunc
 * @param {string} name
 */

/**
 * @type {helloFunc}
 */
module.exports.hello = addon.hello;
