/**
 * Module documentation for TypeScript testing
 */

// @ts-nocheck

import { Component, OnInit } from '@angular/core';
import axios from 'axios';
import * as lodash from 'lodash';

// Regular comment - ignored

/**
 * Adds two numbers
 * @param a First number
 * @param b Second number
 * @returns Sum of a and b
 */
function add(a: number, b: number): number {
    return a + b;
}

/**
 * Multiplies two numbers
 */
const multiply = (a: number, b: number): number => {
    return a * b;
};

/**
 * Async function example
 */
async function fetchData(url: string): Promise<string> {
    const response = await fetch(url);
    return response.json();
}

/**
 * User interface
 */
export interface User {
    name: string;
    age: number;
}

/**
 * User class
 */
export class User {
    private name: string;
    private age: number;

    /**
     * Constructor
     * @param name User name
     * @param age User age
     */
    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    /**
     * Get user name
     * @returns User name
     */
    getName(): string {
        return this.name;
    }
}

/**
 * Service class
 */
export class UserService {
    private users: User[] = [];

    /**
     * Add user method
     * @param user User to add
     */
    addUser(user: User): void {
        this.users.push(user);
    }

    /**
     * Get users method
     * @returns List of users
     */
    getUsers(): User[] {
        return this.users;
    }
}

/**
 * Color enum
 */
export enum Color {
    Red,
    Green,
    Blue
}

/**
 * Result type
 */
export type Result<T> = {
    success: boolean;
    data?: T;
    error?: string;
};

/**
 * Global constants
 */
export const MAX_SIZE: number = 1024;
export const DEFAULT_TIMEOUT: number = 30;

/**
 * Global variables
 */
export let appName: string = "vibe-analyzer";
let counter: number = 0;

// Regular comment at the end - ignored
