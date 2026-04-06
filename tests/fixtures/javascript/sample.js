/**
 * Module documentation for JavaScript testing
 */

import fs from 'fs'
import path from 'path'
import express from 'express'
import { fileURLToPath } from 'url'

// Regular comment - ignored

/**
 * Adds two numbers
 * @param {number} a - First number
 * @param {number} b - Second number
 * @returns {number} Sum of a and b
 */
function add(a, b) {
  return a + b
}

/**
 * Multiplies two numbers
 */
const multiply = (a, b) => {
  return a * b
}

/**
 * Async function example
 */
async function fetchData(url) {
  const response = await fetch(url)
  return response.json()
}

/**
 * User class
 */
class User {
  /**
   * Constructor
   * @param {string} name
   * @param {number} age
   */
  constructor(name, age) {
    this.name = name
    this.age = age
  }

  /**
   * Get user name
   * @returns {string}
   */
  getName() {
    return this.name
  }
}

/**
 * Config object
 */
const config = {
  debug: false,
  maxSize: 1024,
}

// Global constants
export const MAX_SIZE = 1024
export const APP_NAME = 'vibe-analyzer'
let counter = 0

// Regular comment at the end - ignored
