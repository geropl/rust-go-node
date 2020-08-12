/* tslint:disable */
/* eslint-disable */
/**
*/
export class Evaluator {
  free(): void;
/**
* This creates a new Evaluator for the given key and domain to check against
* @param {string} key
* @param {string} domain
* @returns {Evaluator}
*/
  static createFromLicenseKey(key: string, domain: string): Evaluator;
/**
* Validates the given license key. Returns true if the license is valid for the given domain
* @returns {boolean}
*/
  validate(): boolean;
/**
* Returns true if the given license is valid and enables the given feature
* @param {string} feature
* @returns {boolean}
*/
  enabled(feature: string): boolean;
/**
* Returns true if either:
*  - the license has no restrictions on seats
*  - the license permits at least the given number of seats
* @param {number} seats
* @returns {boolean}
*/
  hasEnoughSeats(seats: number): boolean;
/**
* Retrn true if:
*  - the license permits the use of prebuilds
*  - the accumulated time spent doing prebuilds does not exceed the one defined in the license
* @param {BigInt} total_prebuild_time_spent_seconds
* @returns {boolean}
*/
  canUsePrebuild(total_prebuild_time_spent_seconds: BigInt): boolean;
/**
* Returns a string representation of the license (for debugging purposes only)
* @returns {string}
*/
  inspect(): string;
}
