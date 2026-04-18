/// Module documentation for Swift testing
/// This is a module comment

import Foundation
import UIKit

// Regular comment - ignored

/**
 * Adds two numbers
 * - Parameters:
 *   - a: First number
 *   - b: Second number
 * - Returns: Sum of a and b
 */
func add(_ a: Int, _ b: Int) -> Int {
    return a + b
}

/**
 * Multiplies two numbers
 */
func multiply(_ a: Int, _ b: Int) -> Int {
    return a * b
}

/**
 * Async function example
 */
func fetchData(url: String) async -> String {
    return "data"
}

/**
 * User struct
 */
struct User {
    let name: String
    let age: Int

    /**
     * Get user description
     */
    func getDescription() -> String {
        return "\(name), \(age)"
    }
}

/**
 * Service class
 */
class UserService {
    private var users: [User] = []

    /**
     * Add user method
     */
    func addUser(_ user: User) {
        users.append(user)
    }

    /**
     * Get users method
     */
    func getUsers() -> [User] {
        return users
    }
}

/**
 * Color enum
 */
enum Color {
    case red
    case green
    case blue
}

/**
 * Result enum with associated values
 */
enum Result {
    case success(String)
    case error(String)
}

/**
 * Global constants
 */
let MAX_SIZE = 1024
let DEFAULT_TIMEOUT = 30

/**
 * Global variables
 */
var appName = "vibe-analyzer"
var counter = 0

// Regular comment at the end - ignored
